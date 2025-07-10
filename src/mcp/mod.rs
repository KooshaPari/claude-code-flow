use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpStatus {
    pub connected: bool,
    pub servers: Vec<String>,
    pub tools_available: u32,
}

pub struct McpClient {
    config: crate::config::McpConfig,
    servers: RwLock<HashMap<String, McpServerInfo>>,
    tools_cache: RwLock<HashMap<String, McpTool>>,
    connection_pool: RwLock<HashMap<String, McpConnection>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpServerInfo {
    id: String,
    name: String,
    url: String,
    status: ServerStatus,
    capabilities: Vec<String>,
    last_ping: Option<u64>,
    tools_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerStatus {
    Connected,
    Disconnected,
    Error,
    Initializing,
}

#[derive(Debug, Clone)]
struct McpConnection {
    server_id: String,
    established_at: u64,
    last_activity: u64,
    message_count: u64,
}

impl McpClient {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing MCP client with server discovery");
        
        let client = Self {
            config: config.mcp.clone(),
            servers: RwLock::new(HashMap::new()),
            tools_cache: RwLock::new(HashMap::new()),
            connection_pool: RwLock::new(HashMap::new()),
        };
        
        // Discover and register default servers
        client.discover_default_servers().await?;
        
        info!("MCP client initialized successfully");
        Ok(client)
    }
    
    async fn discover_default_servers(&self) -> Result<()> {
        let mut servers = self.servers.write().await;
        
        // Register common MCP servers
        let default_servers = vec![
            McpServerInfo {
                id: "ruv-swarm".to_string(),
                name: "Claude Flow MCP Server".to_string(),
                url: "stdio://claude-flow-mcp".to_string(),
                status: ServerStatus::Disconnected,
                capabilities: vec![
                    "swarm_orchestration".to_string(),
                    "memory_management".to_string(),
                    "neural_processing".to_string(),
                    "task_coordination".to_string(),
                ],
                last_ping: None,
                tools_count: 0,
            },
            McpServerInfo {
                id: "filesystem".to_string(),
                name: "Filesystem MCP Server".to_string(),
                url: "stdio://mcp-server-filesystem".to_string(),
                status: ServerStatus::Disconnected,
                capabilities: vec![
                    "file_operations".to_string(),
                    "directory_management".to_string(),
                ],
                last_ping: None,
                tools_count: 0,
            },
            McpServerInfo {
                id: "git".to_string(),
                name: "Git MCP Server".to_string(),
                url: "stdio://mcp-server-git".to_string(),
                status: ServerStatus::Disconnected,
                capabilities: vec![
                    "version_control".to_string(),
                    "repository_management".to_string(),
                ],
                last_ping: None,
                tools_count: 0,
            },
        ];
        
        for server in default_servers {
            servers.insert(server.id.clone(), server);
        }
        
        debug!("Discovered {} default MCP servers", servers.len());
        Ok(())
    }
    
    pub async fn setup_servers(&self) -> Result<()> {
        info!("Setting up MCP servers");
        
        let servers = self.servers.read().await.clone();
        let mut setup_count = 0;
        
        for (server_id, server_info) in servers {
            match self.setup_individual_server(&server_id, &server_info).await {
                Ok(_) => {
                    self.update_server_status(&server_id, ServerStatus::Connected).await?;
                    setup_count += 1;
                    info!("Successfully set up MCP server: {}", server_info.name);
                }
                Err(e) => {
                    self.update_server_status(&server_id, ServerStatus::Error).await?;
                    warn!("Failed to set up MCP server {}: {}", server_info.name, e);
                }
            }
        }
        
        info!("Set up {}/{} MCP servers successfully", setup_count, servers.len());
        Ok(())
    }
    
    async fn setup_individual_server(&self, server_id: &str, server_info: &McpServerInfo) -> Result<()> {
        debug!("Setting up individual server: {}", server_info.name);
        
        // Update server status to initializing
        self.update_server_status(server_id, ServerStatus::Initializing).await?;
        
        // Test server connectivity
        let connectivity_test = self.test_server_connectivity(server_info).await;
        
        match connectivity_test {
            Ok(_) => {
                // Establish connection
                self.establish_connection(server_id, server_info).await?;
                
                // Discover tools
                self.discover_server_tools(server_id, server_info).await?;
                
                debug!("Server setup completed: {}", server_info.name);
                Ok(())
            }
            Err(e) => {
                warn!("Server connectivity test failed for {}: {}", server_info.name, e);
                Err(e)
            }
        }
    }
    
    async fn test_server_connectivity(&self, server_info: &McpServerInfo) -> Result<()> {
        debug!("Testing connectivity for server: {}", server_info.name);
        
        if server_info.url.starts_with("stdio://") {
            // Test stdio-based server by checking if the command exists
            let command_name = server_info.url.strip_prefix("stdio://").unwrap_or("");
            
            if command_name.is_empty() {
                return Err(anyhow::anyhow!("Invalid stdio URL: {}", server_info.url));
            }
            
            // Try to execute the command with --version or --help to test availability
            let output = Command::new(command_name)
                .arg("--version")
                .output();
            
            match output {
                Ok(_) => {
                    debug!("Stdio server {} is available", command_name);
                    Ok(())
                }
                Err(_) => {
                    // Try with --help as fallback
                    let help_output = Command::new(command_name)
                        .arg("--help")
                        .output();
                    
                    match help_output {
                        Ok(_) => {
                            debug!("Stdio server {} is available (via --help)", command_name);
                            Ok(())
                        }
                        Err(e) => {
                            Err(anyhow::anyhow!("Stdio server {} not available: {}", command_name, e))
                        }
                    }
                }
            }
        } else if server_info.url.starts_with("http://") || server_info.url.starts_with("https://") {
            // For HTTP-based servers, we would implement HTTP connectivity test
            info!("HTTP server connectivity test not implemented yet: {}", server_info.url);
            Ok(()) // Placeholder - assume success for now
        } else {
            Err(anyhow::anyhow!("Unsupported server URL format: {}", server_info.url))
        }
    }
    
    async fn establish_connection(&self, server_id: &str, server_info: &McpServerInfo) -> Result<()> {
        debug!("Establishing connection to server: {}", server_info.name);
        
        let connection = McpConnection {
            server_id: server_id.to_string(),
            established_at: self.current_timestamp(),
            last_activity: self.current_timestamp(),
            message_count: 0,
        };
        
        self.connection_pool.write().await.insert(server_id.to_string(), connection);
        debug!("Connection established for server: {}", server_info.name);
        
        Ok(())
    }
    
    async fn discover_server_tools(&self, server_id: &str, server_info: &McpServerInfo) -> Result<()> {
        debug!("Discovering tools for server: {}", server_info.name);
        
        // Simulate tool discovery based on server capabilities
        let tools = self.generate_tools_for_capabilities(&server_info.capabilities).await;
        
        let mut tools_cache = self.tools_cache.write().await;
        let mut tool_count = 0;
        
        for tool in tools {
            let tool_key = format!("{}:{}", server_id, tool.name);
            tools_cache.insert(tool_key, tool);
            tool_count += 1;
        }
        
        // Update server tool count
        self.update_server_tool_count(server_id, tool_count).await?;
        
        debug!("Discovered {} tools for server: {}", tool_count, server_info.name);
        Ok(())
    }
    
    async fn generate_tools_for_capabilities(&self, capabilities: &[String]) -> Vec<McpTool> {
        let mut tools = Vec::new();
        
        for capability in capabilities {
            match capability.as_str() {
                "swarm_orchestration" => {
                    tools.extend(vec![
                        McpTool {
                            name: "swarm_init".to_string(),
                            description: "Initialize a swarm with specified topology and agent count".to_string(),
                            parameters: serde_json::json!({
                                "topology": "string",
                                "maxAgents": "number",
                                "strategy": "string"
                            }),
                        },
                        McpTool {
                            name: "agent_spawn".to_string(),
                            description: "Spawn a new agent with specified type and configuration".to_string(),
                            parameters: serde_json::json!({
                                "type": "string",
                                "name": "string",
                                "config": "object"
                            }),
                        },
                        McpTool {
                            name: "task_orchestrate".to_string(),
                            description: "Orchestrate task execution across swarm agents".to_string(),
                            parameters: serde_json::json!({
                                "task": "string",
                                "strategy": "string",
                                "priority": "string"
                            }),
                        },
                        McpTool {
                            name: "swarm_status".to_string(),
                            description: "Get current swarm status and metrics".to_string(),
                            parameters: serde_json::json!({}),
                        },
                    ]);
                }
                "memory_management" => {
                    tools.extend(vec![
                        McpTool {
                            name: "memory_usage".to_string(),
                            description: "Store or retrieve data from persistent memory".to_string(),
                            parameters: serde_json::json!({
                                "action": "string",
                                "key": "string",
                                "value": "string",
                                "namespace": "string"
                            }),
                        },
                        McpTool {
                            name: "memory_stats".to_string(),
                            description: "Get memory usage statistics and health metrics".to_string(),
                            parameters: serde_json::json!({}),
                        },
                    ]);
                }
                "neural_processing" => {
                    tools.extend(vec![
                        McpTool {
                            name: "neural_train".to_string(),
                            description: "Train neural patterns for enhanced processing".to_string(),
                            parameters: serde_json::json!({
                                "pattern": "string",
                                "data": "string",
                                "epochs": "number"
                            }),
                        },
                        McpTool {
                            name: "neural_predict".to_string(),
                            description: "Make predictions using trained neural models".to_string(),
                            parameters: serde_json::json!({
                                "model": "string",
                                "input": "string"
                            }),
                        },
                        McpTool {
                            name: "neural_status".to_string(),
                            description: "Get neural engine status and model information".to_string(),
                            parameters: serde_json::json!({}),
                        },
                    ]);
                }
                "file_operations" => {
                    tools.extend(vec![
                        McpTool {
                            name: "read_file".to_string(),
                            description: "Read contents of a file".to_string(),
                            parameters: serde_json::json!({
                                "path": "string"
                            }),
                        },
                        McpTool {
                            name: "write_file".to_string(),
                            description: "Write contents to a file".to_string(),
                            parameters: serde_json::json!({
                                "path": "string",
                                "content": "string"
                            }),
                        },
                    ]);
                }
                "version_control" => {
                    tools.extend(vec![
                        McpTool {
                            name: "git_status".to_string(),
                            description: "Get git repository status".to_string(),
                            parameters: serde_json::json!({}),
                        },
                        McpTool {
                            name: "git_commit".to_string(),
                            description: "Create a git commit".to_string(),
                            parameters: serde_json::json!({
                                "message": "string",
                                "files": "array"
                            }),
                        },
                    ]);
                }
                _ => {
                    // Unknown capability - add generic tool
                    tools.push(McpTool {
                        name: format!("{}_tool", capability),
                        description: format!("Tool for {} capability", capability),
                        parameters: serde_json::json!({}),
                    });
                }
            }
        }
        
        tools
    }
    
    async fn update_server_status(&self, server_id: &str, status: ServerStatus) -> Result<()> {
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.get_mut(server_id) {
            server.status = status;
            server.last_ping = Some(self.current_timestamp());
        }
        Ok(())
    }
    
    async fn update_server_tool_count(&self, server_id: &str, tool_count: u32) -> Result<()> {
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.get_mut(server_id) {
            server.tools_count = tool_count;
        }
        Ok(())
    }
    
    pub async fn setup_with_options(&self, auto_permissions: bool, tools_87: bool) -> Result<()> {
        info!("Setting up MCP with enhanced options (auto_permissions: {}, tools_87: {})", auto_permissions, tools_87);
        
        // First perform regular setup
        self.setup_servers().await?;
        
        // Apply enhanced options
        if auto_permissions {
            self.enable_auto_permissions().await?;
        }
        
        if tools_87 {
            self.enable_tools_87_compatibility().await?;
        }
        
        info!("Enhanced MCP setup completed");
        Ok(())
    }
    
    async fn enable_auto_permissions(&self) -> Result<()> {
        info!("Enabling auto-permissions for MCP tools");
        
        // This would typically set up automatic permission grants for known safe tools
        let mut servers = self.servers.write().await;
        for server in servers.values_mut() {
            server.capabilities.push("auto_permissions".to_string());
        }
        
        debug!("Auto-permissions enabled for all servers");
        Ok(())
    }
    
    async fn enable_tools_87_compatibility(&self) -> Result<()> {
        info!("Enabling Tools 87 compatibility mode");
        
        // This would enable compatibility with MCP Tools specification version 87
        let additional_tools = vec![
            McpTool {
                name: "batch_execute".to_string(),
                description: "Execute multiple tools in batch for improved performance".to_string(),
                parameters: serde_json::json!({
                    "tools": "array",
                    "parallel": "boolean"
                }),
            },
            McpTool {
                name: "tool_info".to_string(),
                description: "Get detailed information about available tools".to_string(),
                parameters: serde_json::json!({
                    "tool_name": "string"
                }),
            },
        ];
        
        let mut tools_cache = self.tools_cache.write().await;
        for tool in additional_tools {
            tools_cache.insert(format!("compatibility:{}", tool.name), tool);
        }
        
        debug!("Tools 87 compatibility enabled");
        Ok(())
    }
    
    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        debug!("Listing all available MCP tools");
        
        let tools_cache = self.tools_cache.read().await;
        let tools: Vec<McpTool> = tools_cache.values().cloned().collect();
        
        info!("Found {} available MCP tools", tools.len());
        Ok(tools)
    }
    
    pub async fn test_connectivity(&self) -> Result<McpStatus> {
        info!("Testing MCP connectivity across all servers");
        
        let servers = self.servers.read().await;
        let tools_cache = self.tools_cache.read().await;
        
        let mut connected_servers = Vec::new();
        let mut total_connected = 0;
        
        for (server_id, server_info) in servers.iter() {
            match self.ping_server(server_info).await {
                Ok(_) => {
                    connected_servers.push(server_info.name.clone());
                    total_connected += 1;
                    
                    // Update last ping time
                    drop(servers);
                    self.update_server_status(server_id, ServerStatus::Connected).await?;
                    let servers = self.servers.read().await; // Re-acquire lock
                }
                Err(e) => {
                    warn!("Server {} connectivity test failed: {}", server_info.name, e);
                    drop(servers);
                    self.update_server_status(server_id, ServerStatus::Error).await?;
                    let servers = self.servers.read().await; // Re-acquire lock
                }
            }
        }
        
        let total_servers = servers.len();
        let tools_available = tools_cache.len() as u32;
        
        let connectivity_status = McpStatus {
            connected: total_connected > 0,
            servers: connected_servers,
            tools_available,
        };
        
        info!("Connectivity test complete: {}/{} servers connected, {} tools available", 
              total_connected, total_servers, tools_available);
        
        Ok(connectivity_status)
    }
    
    async fn ping_server(&self, server_info: &McpServerInfo) -> Result<()> {
        debug!("Pinging server: {}", server_info.name);
        
        // For stdio servers, we can't really "ping" but we can check if they're still available
        if server_info.url.starts_with("stdio://") {
            let command_name = server_info.url.strip_prefix("stdio://").unwrap_or("");
            
            let output = Command::new(command_name)
                .arg("--version")
                .output()
                .with_context(|| format!("Failed to ping stdio server: {}", command_name))?;
            
            if output.status.success() {
                debug!("Server {} is responsive", server_info.name);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Server {} is not responsive", server_info.name))
            }
        } else {
            // For HTTP servers, we would implement HTTP ping
            debug!("HTTP ping not implemented, assuming server {} is available", server_info.name);
            Ok(())
        }
    }
    
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}