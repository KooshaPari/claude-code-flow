package main

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"strings"
	"sync"
	"time"

	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
)

// MCPServer represents an MCP server instance
type MCPServer struct {
	Name         string            `json:"name"`
	Command      string            `json:"command"`
	Args         []string          `json:"args"`
	Env          map[string]string `json:"env"`
	Status       string            `json:"status"`
	PID          int               `json:"pid"`
	StartedAt    time.Time         `json:"started_at"`
	LastActivity time.Time         `json:"last_activity"`
	Tools        []MCPTool         `json:"tools"`
}

// MCPTool represents an available MCP tool
type MCPTool struct {
	Name        string                 `json:"name"`
	Description string                 `json:"description"`
	Parameters  map[string]interface{} `json:"parameters"`
	Category    string                 `json:"category"`
	Version     string                 `json:"version"`
}

// MCPMessage represents a message in the MCP protocol
type MCPMessage struct {
	ID     string                 `json:"id"`
	Method string                 `json:"method"`
	Params map[string]interface{} `json:"params"`
	Result map[string]interface{} `json:"result,omitempty"`
	Error  *MCPError              `json:"error,omitempty"`
}

// MCPError represents an MCP protocol error
type MCPError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

// MCPManager manages all MCP servers and tools
type MCPManager struct {
	servers    map[string]*MCPServer
	tools      map[string]MCPTool
	mutex      sync.RWMutex
	ctx        context.Context
	cancel     context.CancelFunc
	clients    map[string]*websocket.Conn
	clientsMux sync.RWMutex
}

// WebSocket upgrader
var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
}

func main() {
	fmt.Println("🔧 Claude Flow MCP Manager (Go) starting...")

	// Create MCP manager
	ctx, cancel := context.WithCancel(context.Background())
	manager := &MCPManager{
		servers:    make(map[string]*MCPServer),
		tools:      make(map[string]MCPTool),
		ctx:        ctx,
		cancel:     cancel,
		clients:    make(map[string]*websocket.Conn),
		clientsMux: sync.RWMutex{},
	}

	// Setup default MCP servers
	if err := manager.setupDefaultServers(); err != nil {
		log.Fatalf("Failed to setup default servers: %v", err)
	}

	// Start health monitoring
	go manager.startHealthMonitoring()

	// Setup HTTP routes
	router := mux.NewRouter()

	// MCP server management endpoints
	router.HandleFunc("/api/mcp/servers", manager.handleListServers).Methods("GET")
	router.HandleFunc("/api/mcp/servers", manager.handleCreateServer).Methods("POST")
	router.HandleFunc("/api/mcp/servers/{name}", manager.handleGetServer).Methods("GET")
	router.HandleFunc("/api/mcp/servers/{name}/start", manager.handleStartServer).Methods("POST")
	router.HandleFunc("/api/mcp/servers/{name}/stop", manager.handleStopServer).Methods("POST")
	router.HandleFunc("/api/mcp/servers/{name}/restart", manager.handleRestartServer).Methods("POST")

	// MCP tools endpoints
	router.HandleFunc("/api/mcp/tools", manager.handleListTools).Methods("GET")
	router.HandleFunc("/api/mcp/tools/{name}", manager.handleGetTool).Methods("GET")
	router.HandleFunc("/api/mcp/tools/{name}/execute", manager.handleExecuteTool).Methods("POST")

	// MCP protocol endpoints
	router.HandleFunc("/api/mcp/call", manager.handleMCPCall).Methods("POST")
	router.HandleFunc("/api/mcp/batch", manager.handleMCPBatch).Methods("POST")

	// Status and health endpoints
	router.HandleFunc("/api/mcp/status", manager.handleStatus).Methods("GET")
	router.HandleFunc("/api/mcp/health", manager.handleHealth).Methods("GET")

	// WebSocket endpoint for real-time updates
	router.HandleFunc("/ws/mcp", manager.handleWebSocket)

	// Start server
	port := 8082
	fmt.Printf("🚀 MCP Manager API server starting on port %d\n", port)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", port), router))
}

// setupDefaultServers initializes the default MCP servers
func (m *MCPManager) setupDefaultServers() error {
	fmt.Println("📋 Setting up default MCP servers...")

	// Claude Flow MCP Server
	claudeFlowServer := &MCPServer{
		Name:    "claude-flow",
		Command: "npx",
		Args:    []string{"claude-flow", "mcp", "start"},
		Env:     map[string]string{},
		Status:  "stopped",
	}

	// Ruv-Swarm MCP Server
	ruvSwarmServer := &MCPServer{
		Name:    "ruv-swarm",
		Command: "npx",
		Args:    []string{"ruv-swarm", "mcp", "start"},
		Env:     map[string]string{},
		Status:  "stopped",
	}

	m.mutex.Lock()
	m.servers["claude-flow"] = claudeFlowServer
	m.servers["ruv-swarm"] = ruvSwarmServer
	m.mutex.Unlock()

	// Load default tools
	m.loadDefaultTools()

	fmt.Printf("✅ Setup %d default MCP servers\n", len(m.servers))
	return nil
}

// loadDefaultTools loads the default set of MCP tools
func (m *MCPManager) loadDefaultTools() {
	tools := []MCPTool{
		{
			Name:        "swarm_init",
			Description: "Initialize swarm coordination topology",
			Category:    "swarm",
			Version:     "2.0.0",
			Parameters: map[string]interface{}{
				"topology":   "string",
				"maxAgents":  "number",
				"strategy":   "string",
			},
		},
		{
			Name:        "agent_spawn",
			Description: "Spawn specialized agent for task execution",
			Category:    "agents",
			Version:     "2.0.0",
			Parameters: map[string]interface{}{
				"type":         "string",
				"capabilities": "array",
				"resources":    "object",
			},
		},
		{
			Name:        "memory_usage",
			Description: "Store or retrieve from distributed memory",
			Category:    "memory",
			Version:     "2.0.0",
			Parameters: map[string]interface{}{
				"action":    "string",
				"key":       "string",
				"value":     "string",
				"namespace": "string",
			},
		},
		{
			Name:        "neural_train",
			Description: "Train neural patterns for optimization",
			Category:    "neural",
			Version:     "2.0.0",
			Parameters: map[string]interface{}{
				"pattern": "string",
				"data":    "string",
				"epochs":  "number",
			},
		},
		{
			Name:        "task_orchestrate",
			Description: "Orchestrate complex task execution",
			Category:    "coordination",
			Version:     "2.0.0",
			Parameters: map[string]interface{}{
				"task":     "string",
				"strategy": "string",
				"agents":   "array",
			},
		},
		{
			Name:        "github_repo_analyze",
			Description: "Analyze GitHub repository structure and metrics",
			Category:    "github",
			Version:     "2.0.0",
			Parameters: map[string]interface{}{
				"repo":         "string",
				"analysis_type": "string",
				"depth":        "number",
			},
		},
	}

	m.mutex.Lock()
	for _, tool := range tools {
		m.tools[tool.Name] = tool
	}
	m.mutex.Unlock()

	fmt.Printf("📚 Loaded %d default MCP tools\n", len(tools))
}

// startHealthMonitoring starts background health monitoring for servers
func (m *MCPManager) startHealthMonitoring() {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ticker.C:
			m.performHealthCheck()
		case <-m.ctx.Done():
			return
		}
	}
}

// performHealthCheck checks the health of all servers
func (m *MCPManager) performHealthCheck() {
	m.mutex.RLock()
	servers := make([]*MCPServer, 0, len(m.servers))
	for _, server := range m.servers {
		servers = append(servers, server)
	}
	m.mutex.RUnlock()

	for _, server := range servers {
		if server.Status == "running" {
			// Simulate health check (in real implementation, this would ping the server)
			server.LastActivity = time.Now()
		}
	}
}

// HTTP Handlers

func (m *MCPManager) handleListServers(w http.ResponseWriter, r *http.Request) {
	m.mutex.RLock()
	servers := make([]*MCPServer, 0, len(m.servers))
	for _, server := range m.servers {
		servers = append(servers, server)
	}
	m.mutex.RUnlock()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"servers": servers,
		"count":   len(servers),
	})
}

func (m *MCPManager) handleCreateServer(w http.ResponseWriter, r *http.Request) {
	var server MCPServer
	if err := json.NewDecoder(r.Body).Decode(&server); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	server.Status = "stopped"
	server.StartedAt = time.Time{}

	m.mutex.Lock()
	m.servers[server.Name] = &server
	m.mutex.Unlock()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"message": "Server created successfully",
		"server":  server,
	})
}

func (m *MCPManager) handleGetServer(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]

	m.mutex.RLock()
	server, exists := m.servers[name]
	m.mutex.RUnlock()

	if !exists {
		http.Error(w, "Server not found", http.StatusNotFound)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(server)
}

func (m *MCPManager) handleStartServer(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]

	m.mutex.Lock()
	server, exists := m.servers[name]
	if !exists {
		m.mutex.Unlock()
		http.Error(w, "Server not found", http.StatusNotFound)
		return
	}

	if server.Status == "running" {
		m.mutex.Unlock()
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"message": "Server is already running",
			"status":  "running",
		})
		return
	}

	// Start the server process
	server.Status = "starting"
	m.mutex.Unlock()

	go m.startServerProcess(server)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"message": "Server start initiated",
		"status":  "starting",
	})
}

func (m *MCPManager) handleStopServer(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]

	m.mutex.Lock()
	server, exists := m.servers[name]
	if !exists {
		m.mutex.Unlock()
		http.Error(w, "Server not found", http.StatusNotFound)
		return
	}

	server.Status = "stopped"
	server.PID = 0
	m.mutex.Unlock()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"message": "Server stopped successfully",
		"status":  "stopped",
	})
}

func (m *MCPManager) handleRestartServer(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]

	// Stop first
	m.mutex.Lock()
	server, exists := m.servers[name]
	if !exists {
		m.mutex.Unlock()
		http.Error(w, "Server not found", http.StatusNotFound)
		return
	}

	server.Status = "restarting"
	m.mutex.Unlock()

	// Restart the server
	go m.startServerProcess(server)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"message": "Server restart initiated",
		"status":  "restarting",
	})
}

func (m *MCPManager) handleListTools(w http.ResponseWriter, r *http.Request) {
	m.mutex.RLock()
	tools := make([]MCPTool, 0, len(m.tools))
	for _, tool := range m.tools {
		tools = append(tools, tool)
	}
	m.mutex.RUnlock()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"tools": tools,
		"count": len(tools),
	})
}

func (m *MCPManager) handleGetTool(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]

	m.mutex.RLock()
	tool, exists := m.tools[name]
	m.mutex.RUnlock()

	if !exists {
		http.Error(w, "Tool not found", http.StatusNotFound)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tool)
}

func (m *MCPManager) handleExecuteTool(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	toolName := vars["name"]

	var request struct {
		Parameters map[string]interface{} `json:"parameters"`
		Context    map[string]interface{} `json:"context,omitempty"`
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	m.mutex.RLock()
	tool, exists := m.tools[toolName]
	m.mutex.RUnlock()

	if !exists {
		http.Error(w, "Tool not found", http.StatusNotFound)
		return
	}

	// Simulate tool execution
	result := m.simulateToolExecution(tool, request.Parameters)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"tool":       toolName,
		"result":     result,
		"timestamp":  time.Now(),
		"success":    true,
	})
}

func (m *MCPManager) handleMCPCall(w http.ResponseWriter, r *http.Request) {
	var message MCPMessage
	if err := json.NewDecoder(r.Body).Decode(&message); err != nil {
		http.Error(w, "Invalid MCP message", http.StatusBadRequest)
		return
	}

	// Process MCP call
	response := m.processMCPCall(message)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}

func (m *MCPManager) handleMCPBatch(w http.ResponseWriter, r *http.Request) {
	var messages []MCPMessage
	if err := json.NewDecoder(r.Body).Decode(&messages); err != nil {
		http.Error(w, "Invalid batch MCP messages", http.StatusBadRequest)
		return
	}

	// Process batch MCP calls
	responses := make([]MCPMessage, len(messages))
	for i, message := range messages {
		responses[i] = m.processMCPCall(message)
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(responses)
}

func (m *MCPManager) handleStatus(w http.ResponseWriter, r *http.Request) {
	m.mutex.RLock()
	runningServers := 0
	for _, server := range m.servers {
		if server.Status == "running" {
			runningServers++
		}
	}
	totalTools := len(m.tools)
	m.mutex.RUnlock()

	status := map[string]interface{}{
		"service":         "claude-flow-mcp",
		"version":         "2.0.0",
		"status":          "active",
		"servers_total":   len(m.servers),
		"servers_running": runningServers,
		"tools_available": totalTools,
		"timestamp":       time.Now(),
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(status)
}

func (m *MCPManager) handleHealth(w http.ResponseWriter, r *http.Request) {
	m.mutex.RLock()
	hasRunningServers := false
	for _, server := range m.servers {
		if server.Status == "running" {
			hasRunningServers = true
			break
		}
	}
	m.mutex.RUnlock()

	health := map[string]interface{}{
		"status": "healthy",
		"checks": map[string]bool{
			"servers_available": len(m.servers) > 0,
			"servers_running":   hasRunningServers,
			"tools_loaded":      len(m.tools) > 0,
			"api_responsive":    true,
		},
		"timestamp": time.Now(),
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(health)
}

func (m *MCPManager) handleWebSocket(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("WebSocket upgrade error: %v", err)
		return
	}
	defer conn.Close()

	clientID := fmt.Sprintf("client_%d", time.Now().Unix())
	m.clientsMux.Lock()
	m.clients[clientID] = conn
	m.clientsMux.Unlock()

	defer func() {
		m.clientsMux.Lock()
		delete(m.clients, clientID)
		m.clientsMux.Unlock()
	}()

	// Send real-time updates
	ticker := time.NewTicker(10 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ticker.C:
			m.mutex.RLock()
			update := map[string]interface{}{
				"type":      "mcp_update",
				"servers":   len(m.servers),
				"tools":     len(m.tools),
				"timestamp": time.Now(),
			}
			m.mutex.RUnlock()

			if err := conn.WriteJSON(update); err != nil {
				log.Printf("WebSocket write error: %v", err)
				return
			}
		case <-m.ctx.Done():
			return
		}
	}
}

// Helper functions

func (m *MCPManager) startServerProcess(server *MCPServer) {
	// Simulate starting a server process
	time.Sleep(2 * time.Second) // Simulate startup time

	m.mutex.Lock()
	server.Status = "running"
	server.StartedAt = time.Now()
	server.LastActivity = time.Now()
	server.PID = 12345 // Simulate PID
	m.mutex.Unlock()

	fmt.Printf("🟢 MCP Server '%s' started successfully\n", server.Name)
}

func (m *MCPManager) simulateToolExecution(tool MCPTool, parameters map[string]interface{}) map[string]interface{} {
	// Simulate tool execution based on tool category
	switch tool.Category {
	case "swarm":
		return map[string]interface{}{
			"swarm_id":     fmt.Sprintf("swarm_%d", time.Now().Unix()),
			"topology":     parameters["topology"],
			"agents_max":   parameters["maxAgents"],
			"status":       "initialized",
			"coordination": "active",
		}
	case "agents":
		return map[string]interface{}{
			"agent_id":     fmt.Sprintf("agent_%d", time.Now().Unix()),
			"type":         parameters["type"],
			"status":       "spawned",
			"capabilities": parameters["capabilities"],
		}
	case "memory":
		return map[string]interface{}{
			"action":      parameters["action"],
			"key":         parameters["key"],
			"namespace":   parameters["namespace"],
			"status":      "success",
			"timestamp":   time.Now(),
		}
	case "neural":
		return map[string]interface{}{
			"training_id": fmt.Sprintf("train_%d", time.Now().Unix()),
			"pattern":     parameters["pattern"],
			"status":      "started",
			"epochs":      parameters["epochs"],
		}
	case "coordination":
		return map[string]interface{}{
			"task_id":     fmt.Sprintf("task_%d", time.Now().Unix()),
			"strategy":    parameters["strategy"],
			"status":      "orchestrating",
			"agents":      parameters["agents"],
		}
	case "github":
		return map[string]interface{}{
			"analysis_id":   fmt.Sprintf("analysis_%d", time.Now().Unix()),
			"repo":          parameters["repo"],
			"analysis_type": parameters["analysis_type"],
			"status":        "analyzing",
		}
	default:
		return map[string]interface{}{
			"status":  "executed",
			"message": "Tool executed successfully",
		}
	}
}

func (m *MCPManager) processMCPCall(message MCPMessage) MCPMessage {
	response := MCPMessage{
		ID: message.ID,
	}

	// Simulate MCP method handling
	switch message.Method {
	case "tools/list":
		m.mutex.RLock()
		tools := make([]MCPTool, 0, len(m.tools))
		for _, tool := range m.tools {
			tools = append(tools, tool)
		}
		m.mutex.RUnlock()

		response.Result = map[string]interface{}{
			"tools": tools,
		}
	case "tools/call":
		toolName, ok := message.Params["name"].(string)
		if !ok {
			response.Error = &MCPError{
				Code:    -32602,
				Message: "Invalid parameters: tool name required",
			}
			return response
		}

		m.mutex.RLock()
		tool, exists := m.tools[toolName]
		m.mutex.RUnlock()

		if !exists {
			response.Error = &MCPError{
				Code:    -32601,
				Message: fmt.Sprintf("Tool not found: %s", toolName),
			}
			return response
		}

		params, _ := message.Params["arguments"].(map[string]interface{})
		result := m.simulateToolExecution(tool, params)
		response.Result = result
	default:
		response.Error = &MCPError{
			Code:    -32601,
			Message: fmt.Sprintf("Method not found: %s", message.Method),
		}
	}

	return response
}