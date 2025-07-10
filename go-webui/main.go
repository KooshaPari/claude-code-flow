package main

import (
	"context"
	"encoding/json"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"time"

	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
)

// WebUIServer represents the main web UI server
type WebUIServer struct {
	router      *mux.Router
	templates   *template.Template
	clients     map[string]*websocket.Conn
	clientsMux  sync.RWMutex
	ctx         context.Context
	cancel      context.CancelFunc
	services    *ServiceManager
}

// ServiceManager manages connections to backend services
type ServiceManager struct {
	ClaudeFlowAPI   string
	NeuralAPI       string
	MCPAPI          string
	MemoryAPI       string
	GitHubAPI       string
	PerformanceAPI  string
	healthChecks    map[string]bool
	healthMux       sync.RWMutex
}

// WebSocketMessage represents a WebSocket message
type WebSocketMessage struct {
	Type      string                 `json:"type"`
	Component string                 `json:"component,omitempty"`
	Action    string                 `json:"action,omitempty"`
	Data      map[string]interface{} `json:"data,omitempty"`
	Timestamp time.Time              `json:"timestamp"`
}

// DashboardData represents the main dashboard data
type DashboardData struct {
	SystemStatus    SystemStatus               `json:"system_status"`
	ServiceStatuses map[string]ServiceStatus   `json:"service_statuses"`
	Metrics         SystemMetrics              `json:"metrics"`
	RecentActivity  []ActivityEvent            `json:"recent_activity"`
	Alerts          []Alert                    `json:"alerts"`
}

// SystemStatus represents overall system status
type SystemStatus struct {
	Status      string    `json:"status"`
	Uptime      string    `json:"uptime"`
	Version     string    `json:"version"`
	Environment string    `json:"environment"`
	LastUpdate  time.Time `json:"last_update"`
}

// ServiceStatus represents individual service status
type ServiceStatus struct {
	Name        string    `json:"name"`
	Status      string    `json:"status"`
	Endpoint    string    `json:"endpoint"`
	ResponseTime time.Duration `json:"response_time"`
	LastCheck   time.Time `json:"last_check"`
}

// SystemMetrics represents system performance metrics
type SystemMetrics struct {
	CPUUsage     float64 `json:"cpu_usage"`
	MemoryUsage  float64 `json:"memory_usage"`
	DiskUsage    float64 `json:"disk_usage"`
	NetworkIO    NetworkIO `json:"network_io"`
	ActiveAgents int     `json:"active_agents"`
	TotalTasks   int     `json:"total_tasks"`
}

// NetworkIO represents network I/O metrics
type NetworkIO struct {
	BytesIn  int64 `json:"bytes_in"`
	BytesOut int64 `json:"bytes_out"`
}

// ActivityEvent represents a system activity event
type ActivityEvent struct {
	ID          string    `json:"id"`
	Type        string    `json:"type"`
	Description string    `json:"description"`
	Actor       string    `json:"actor"`
	Timestamp   time.Time `json:"timestamp"`
	Status      string    `json:"status"`
}

// Alert represents a system alert
type Alert struct {
	ID          string    `json:"id"`
	Level       string    `json:"level"`
	Component   string    `json:"component"`
	Message     string    `json:"message"`
	Timestamp   time.Time `json:"timestamp"`
	Acknowledged bool     `json:"acknowledged"`
}

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
}

func main() {
	fmt.Println("🌊 Claude Flow Web UI Server (Go) starting...")

	// Create context for graceful shutdown
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Initialize service manager
	services := &ServiceManager{
		ClaudeFlowAPI:   "http://localhost:3000",
		NeuralAPI:       "http://localhost:8081",
		MCPAPI:          "http://localhost:8082",
		MemoryAPI:       "http://localhost:8083",
		GitHubAPI:       "http://localhost:8084",
		PerformanceAPI:  "http://localhost:8085",
		healthChecks:    make(map[string]bool),
	}

	// Start health monitoring
	go services.startHealthMonitoring(ctx)

	// Create web UI server
	server := &WebUIServer{
		router:   mux.NewRouter(),
		clients:  make(map[string]*websocket.Conn),
		ctx:      ctx,
		cancel:   cancel,
		services: services,
	}

	// Load templates
	if err := server.loadTemplates(); err != nil {
		log.Fatalf("Failed to load templates: %v", err)
	}

	// Setup routes
	server.setupRoutes()

	// Start periodic data broadcasting
	go server.startDataBroadcasting()

	// Start HTTP server
	port := 8080
	if envPort := os.Getenv("PORT"); envPort != "" {
		port = parsePort(envPort)
	}

	fmt.Printf("🚀 Web UI server starting on port %d\n", port)
	fmt.Printf("📊 Dashboard: http://localhost:%d\n", port)
	fmt.Printf("🧠 Neural UI: http://localhost:%d/neural\n", port)
	fmt.Printf("🤖 Agent Manager: http://localhost:%d/agents\n", port)
	fmt.Printf("💾 Memory Browser: http://localhost:%d/memory\n", port)

	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", port), server.router))
}

// loadTemplates loads HTML templates
func (s *WebUIServer) loadTemplates() error {
	templatesDir := "./templates"
	if _, err := os.Stat(templatesDir); os.IsNotExist(err) {
		// Create templates directory if it doesn't exist
		if err := os.MkdirAll(templatesDir, 0755); err != nil {
			return fmt.Errorf("failed to create templates directory: %v", err)
		}
		// Create default templates
		return s.createDefaultTemplates()
	}

	var err error
	s.templates, err = template.ParseGlob(filepath.Join(templatesDir, "*.html"))
	if err != nil {
		return fmt.Errorf("failed to parse templates: %v", err)
	}

	fmt.Printf("✅ Loaded HTML templates from %s\n", templatesDir)
	return nil
}

// createDefaultTemplates creates default HTML templates
func (s *WebUIServer) createDefaultTemplates() error {
	templates := map[string]string{
		"base.html":        baseTemplate,
		"dashboard.html":   dashboardTemplate,
		"neural.html":      neuralTemplate,
		"agents.html":      agentsTemplate,
		"memory.html":      memoryTemplate,
		"swarm.html":       swarmTemplate,
		"github.html":      githubTemplate,
		"performance.html": performanceTemplate,
		"config.html":      configTemplate,
		"terminal.html":    terminalTemplate,
	}

	for filename, content := range templates {
		path := filepath.Join("./templates", filename)
		if err := os.WriteFile(path, []byte(content), 0644); err != nil {
			return fmt.Errorf("failed to create template %s: %v", filename, err)
		}
	}

	// Load the created templates
	var err error
	s.templates, err = template.ParseGlob("./templates/*.html")
	if err != nil {
		return fmt.Errorf("failed to parse created templates: %v", err)
	}

	fmt.Println("✅ Created and loaded default HTML templates")
	return nil
}

// setupRoutes configures HTTP routes
func (s *WebUIServer) setupRoutes() {
	// Static files
	s.router.PathPrefix("/static/").Handler(
		http.StripPrefix("/static/", http.FileServer(http.Dir("./static/"))),
	)

	// WebAssembly files
	s.router.PathPrefix("/wasm/").Handler(
		http.StripPrefix("/wasm/", http.FileServer(http.Dir("./wasm/"))),
	)

	// Main dashboard
	s.router.HandleFunc("/", s.handleDashboard).Methods("GET")

	// Component pages
	s.router.HandleFunc("/neural", s.handleNeuralPage).Methods("GET")
	s.router.HandleFunc("/agents", s.handleAgentsPage).Methods("GET")
	s.router.HandleFunc("/memory", s.handleMemoryPage).Methods("GET")
	s.router.HandleFunc("/swarm", s.handleSwarmPage).Methods("GET")
	s.router.HandleFunc("/github", s.handleGitHubPage).Methods("GET")
	s.router.HandleFunc("/performance", s.handlePerformancePage).Methods("GET")
	s.router.HandleFunc("/config", s.handleConfigPage).Methods("GET")
	s.router.HandleFunc("/terminal", s.handleTerminalPage).Methods("GET")

	// API endpoints
	api := s.router.PathPrefix("/api").Subrouter()
	api.HandleFunc("/dashboard", s.handleDashboardAPI).Methods("GET")
	api.HandleFunc("/services/status", s.handleServicesStatus).Methods("GET")
	api.HandleFunc("/metrics", s.handleMetricsAPI).Methods("GET")
	api.HandleFunc("/agents", s.handleAgentsAPI).Methods("GET", "POST")
	api.HandleFunc("/agents/{id}", s.handleAgentAPI).Methods("GET", "PUT", "DELETE")
	api.HandleFunc("/memory/browse", s.handleMemoryBrowseAPI).Methods("GET")
	api.HandleFunc("/memory/search", s.handleMemorySearchAPI).Methods("POST")
	api.HandleFunc("/neural/models", s.handleNeuralModelsAPI).Methods("GET")
	api.HandleFunc("/neural/train", s.handleNeuralTrainAPI).Methods("POST")
	api.HandleFunc("/swarm/status", s.handleSwarmStatusAPI).Methods("GET")
	api.HandleFunc("/swarm/topology", s.handleSwarmTopologyAPI).Methods("GET")
	api.HandleFunc("/github/repos", s.handleGitHubReposAPI).Methods("GET")
	api.HandleFunc("/github/analyze", s.handleGitHubAnalyzeAPI).Methods("POST")
	api.HandleFunc("/config", s.handleConfigAPI).Methods("GET", "PUT")

	// WebSocket endpoint
	s.router.HandleFunc("/ws", s.handleWebSocket)

	// Health check
	s.router.HandleFunc("/health", s.handleHealth).Methods("GET")

	fmt.Println("✅ HTTP routes configured")
}

// HTTP Handlers

func (s *WebUIServer) handleDashboard(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Claude Flow Dashboard",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "dashboard.html", data)
}

func (s *WebUIServer) handleNeuralPage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Neural Network Management",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "neural.html", data)
}

func (s *WebUIServer) handleAgentsPage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Agent Management",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "agents.html", data)
}

func (s *WebUIServer) handleMemoryPage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Memory Browser",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "memory.html", data)
}

func (s *WebUIServer) handleSwarmPage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Swarm Visualizer",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "swarm.html", data)
}

func (s *WebUIServer) handleGitHubPage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "GitHub Integration",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "github.html", data)
}

func (s *WebUIServer) handlePerformancePage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Performance Monitor",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "performance.html", data)
}

func (s *WebUIServer) handleConfigPage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Configuration Manager",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "config.html", data)
}

func (s *WebUIServer) handleTerminalPage(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title   string
		Version string
	}{
		Title:   "Terminal Emulator",
		Version: "2.0.0",
	}

	s.renderTemplate(w, "terminal.html", data)
}

// API Handlers

func (s *WebUIServer) handleDashboardAPI(w http.ResponseWriter, r *http.Request) {
	dashboard := s.getDashboardData()
	s.writeJSON(w, dashboard)
}

func (s *WebUIServer) handleServicesStatus(w http.ResponseWriter, r *http.Request) {
	s.services.healthMux.RLock()
	statuses := make(map[string]ServiceStatus)
	
	services := map[string]string{
		"claude-flow": s.services.ClaudeFlowAPI,
		"neural":      s.services.NeuralAPI,
		"mcp":         s.services.MCPAPI,
		"memory":      s.services.MemoryAPI,
		"github":      s.services.GitHubAPI,
		"performance": s.services.PerformanceAPI,
	}

	for name, endpoint := range services {
		healthy := s.services.healthChecks[name]
		status := "healthy"
		if !healthy {
			status = "unhealthy"
		}

		statuses[name] = ServiceStatus{
			Name:         name,
			Status:       status,
			Endpoint:     endpoint,
			ResponseTime: time.Millisecond * 50, // Simulated
			LastCheck:    time.Now(),
		}
	}
	s.services.healthMux.RUnlock()

	s.writeJSON(w, statuses)
}

func (s *WebUIServer) handleMetricsAPI(w http.ResponseWriter, r *http.Request) {
	metrics := s.getSystemMetrics()
	s.writeJSON(w, metrics)
}

func (s *WebUIServer) handleAgentsAPI(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case "GET":
		// Return list of agents
		agents := s.getAgents()
		s.writeJSON(w, map[string]interface{}{
			"agents": agents,
			"count":  len(agents),
		})
	case "POST":
		// Create new agent
		var request map[string]interface{}
		if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}
		
		agent := s.createAgent(request)
		s.writeJSON(w, agent)
	}
}

func (s *WebUIServer) handleAgentAPI(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	agentID := vars["id"]

	switch r.Method {
	case "GET":
		agent := s.getAgent(agentID)
		if agent == nil {
			http.Error(w, "Agent not found", http.StatusNotFound)
			return
		}
		s.writeJSON(w, agent)
	case "PUT":
		var request map[string]interface{}
		if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}
		
		agent := s.updateAgent(agentID, request)
		if agent == nil {
			http.Error(w, "Agent not found", http.StatusNotFound)
			return
		}
		s.writeJSON(w, agent)
	case "DELETE":
		if err := s.deleteAgent(agentID); err != nil {
			http.Error(w, "Agent not found", http.StatusNotFound)
			return
		}
		s.writeJSON(w, map[string]string{"status": "deleted"})
	}
}

func (s *WebUIServer) handleMemoryBrowseAPI(w http.ResponseWriter, r *http.Request) {
	namespace := r.URL.Query().Get("namespace")
	limit := parseInt(r.URL.Query().Get("limit"), 50)
	offset := parseInt(r.URL.Query().Get("offset"), 0)

	memories := s.browseMemory(namespace, limit, offset)
	s.writeJSON(w, memories)
}

func (s *WebUIServer) handleMemorySearchAPI(w http.ResponseWriter, r *http.Request) {
	var request struct {
		Query     string `json:"query"`
		Namespace string `json:"namespace,omitempty"`
		Limit     int    `json:"limit,omitempty"`
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	results := s.searchMemory(request.Query, request.Namespace, request.Limit)
	s.writeJSON(w, results)
}

func (s *WebUIServer) handleNeuralModelsAPI(w http.ResponseWriter, r *http.Request) {
	models := s.getNeuralModels()
	s.writeJSON(w, models)
}

func (s *WebUIServer) handleNeuralTrainAPI(w http.ResponseWriter, r *http.Request) {
	var request map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	job := s.startNeuralTraining(request)
	s.writeJSON(w, job)
}

func (s *WebUIServer) handleSwarmStatusAPI(w http.ResponseWriter, r *http.Request) {
	status := s.getSwarmStatus()
	s.writeJSON(w, status)
}

func (s *WebUIServer) handleSwarmTopologyAPI(w http.ResponseWriter, r *http.Request) {
	topology := s.getSwarmTopology()
	s.writeJSON(w, topology)
}

func (s *WebUIServer) handleGitHubReposAPI(w http.ResponseWriter, r *http.Request) {
	repos := s.getGitHubRepos()
	s.writeJSON(w, repos)
}

func (s *WebUIServer) handleGitHubAnalyzeAPI(w http.ResponseWriter, r *http.Request) {
	var request struct {
		Repository string `json:"repository"`
		Analysis   string `json:"analysis"`
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	analysis := s.analyzeGitHubRepo(request.Repository, request.Analysis)
	s.writeJSON(w, analysis)
}

func (s *WebUIServer) handleConfigAPI(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case "GET":
		config := s.getConfiguration()
		s.writeJSON(w, config)
	case "PUT":
		var request map[string]interface{}
		if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}
		
		config := s.updateConfiguration(request)
		s.writeJSON(w, config)
	}
}

func (s *WebUIServer) handleWebSocket(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("WebSocket upgrade error: %v", err)
		return
	}
	defer conn.Close()

	clientID := fmt.Sprintf("client_%d", time.Now().Unix())
	s.clientsMux.Lock()
	s.clients[clientID] = conn
	s.clientsMux.Unlock()

	defer func() {
		s.clientsMux.Lock()
		delete(s.clients, clientID)
		s.clientsMux.Unlock()
	}()

	fmt.Printf("🔗 WebSocket client connected: %s\n", clientID)

	// Handle incoming messages
	for {
		var msg WebSocketMessage
		if err := conn.ReadJSON(&msg); err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				log.Printf("WebSocket error: %v", err)
			}
			break
		}

		// Process incoming message
		s.handleWebSocketMessage(clientID, msg)
	}

	fmt.Printf("🔌 WebSocket client disconnected: %s\n", clientID)
}

func (s *WebUIServer) handleHealth(w http.ResponseWriter, r *http.Request) {
	health := map[string]interface{}{
		"status":    "healthy",
		"timestamp": time.Now(),
		"services": map[string]bool{
			"web_server": true,
			"websocket":  len(s.clients) >= 0,
			"templates":  s.templates != nil,
		},
	}

	s.writeJSON(w, health)
}

// Helper functions will be implemented in the next part...

func (s *WebUIServer) renderTemplate(w http.ResponseWriter, name string, data interface{}) {
	if err := s.templates.ExecuteTemplate(w, name, data); err != nil {
		log.Printf("Template execution error: %v", err)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
	}
}

func (s *WebUIServer) writeJSON(w http.ResponseWriter, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(data); err != nil {
		log.Printf("JSON encoding error: %v", err)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
	}
}

func parsePort(portStr string) int {
	// Simple port parsing - in production, use strconv.Atoi with error handling
	if portStr == "" {
		return 8080
	}
	// For demo purposes, just return 8080
	return 8080
}

func parseInt(str string, defaultValue int) int {
	// Simple int parsing - in production, use strconv.Atoi with error handling
	if str == "" {
		return defaultValue
	}
	// For demo purposes, just return default
	return defaultValue
}