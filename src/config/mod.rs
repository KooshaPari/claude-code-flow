use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub core: CoreConfig,
    pub agents: AgentConfig,
    pub memory: MemoryConfig,
    pub neural: NeuralConfig,
    pub swarm: SwarmConfig,
    pub github: GithubConfig,
    pub mcp: McpConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    pub log_level: String,
    pub max_concurrent_tasks: u32,
    pub session_timeout: u64,
    pub auto_save_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub max_agents: u32,
    pub default_timeout: u64,
    pub memory_limit_mb: u32,
    pub cpu_limit_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub backend: String, // "sqlite", "json", "memory"
    pub database_path: String,
    pub max_entries: u64,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralConfig {
    pub enabled: bool,
    pub model_path: String,
    pub max_models: u32,
    pub training_enabled: bool,
    pub gpu_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub default_topology: String,
    pub max_swarm_size: u32,
    pub coordination_timeout: u64,
    pub load_balancing_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubConfig {
    pub enabled: bool,
    pub token: Option<String>,
    pub default_org: Option<String>,
    pub webhook_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub enabled: bool,
    pub servers: Vec<McpServerConfig>,
    pub tools_enabled: Vec<String>,
    pub auto_permissions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub terminal_ui_enabled: bool,
    pub web_ui_enabled: bool,
    pub web_ui_port: u16,
    pub real_time_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidation {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "2.0.0".to_string(),
            core: CoreConfig {
                log_level: "info".to_string(),
                max_concurrent_tasks: 10,
                session_timeout: 3600,
                auto_save_interval: 300,
            },
            agents: AgentConfig {
                max_agents: 50,
                default_timeout: 300,
                memory_limit_mb: 512,
                cpu_limit_percent: 50.0,
            },
            memory: MemoryConfig {
                backend: "sqlite".to_string(),
                database_path: "~/.claude-flow/memory.db".to_string(),
                max_entries: 100000,
                compression_enabled: true,
                encryption_enabled: false,
            },
            neural: NeuralConfig {
                enabled: false,
                model_path: "~/.claude-flow/models".to_string(),
                max_models: 10,
                training_enabled: false,
                gpu_enabled: false,
            },
            swarm: SwarmConfig {
                default_topology: "hierarchical".to_string(),
                max_swarm_size: 20,
                coordination_timeout: 60,
                load_balancing_enabled: true,
            },
            github: GithubConfig {
                enabled: false,
                token: None,
                default_org: None,
                webhook_secret: None,
            },
            mcp: McpConfig {
                enabled: true,
                servers: vec![
                    McpServerConfig {
                        name: "claude-flow".to_string(),
                        command: "npx".to_string(),
                        args: vec!["claude-flow".to_string(), "mcp".to_string(), "start".to_string()],
                        env: std::collections::HashMap::new(),
                    },
                    McpServerConfig {
                        name: "ruv-swarm".to_string(),
                        command: "npx".to_string(),
                        args: vec!["ruv-swarm".to_string(), "mcp".to_string(), "start".to_string()],
                        env: std::collections::HashMap::new(),
                    },
                ],
                tools_enabled: vec!["all".to_string()],
                auto_permissions: true,
            },
            ui: UiConfig {
                terminal_ui_enabled: true,
                web_ui_enabled: true,
                web_ui_port: 8080,
                real_time_updates: true,
            },
        }
    }
}

impl Config {
    pub async fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            info!("Loading configuration from: {}", path.display());
            let content = fs::read_to_string(path).await
                .with_context(|| format!("Failed to read config file: {}", path.display()))?;
            
            let config: Config = toml::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
            
            info!("Configuration loaded successfully");
            Ok(config)
        } else {
            warn!("Config file not found, using defaults: {}", path.display());
            let config = Config::default();
            
            // Create config directory if it doesn't exist
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await
                    .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
            }
            
            // Save default configuration
            config.save(path).await?;
            info!("Default configuration saved to: {}", path.display());
            
            Ok(config)
        }
    }
    
    pub async fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;
        
        fs::write(path, content).await
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        
        info!("Configuration saved to: {}", path.display());
        Ok(())
    }
    
    pub fn is_initialized(&self) -> bool {
        // Check if the config appears to be properly initialized
        !self.core.log_level.is_empty() && self.core.max_concurrent_tasks > 0
    }
    
    pub async fn validate(&self) -> Result<ConfigValidation> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Validate core config
        if self.core.max_concurrent_tasks == 0 {
            errors.push("core.max_concurrent_tasks must be greater than 0".to_string());
        }
        
        if self.core.session_timeout == 0 {
            errors.push("core.session_timeout must be greater than 0".to_string());
        }
        
        // Validate agent config
        if self.agents.max_agents == 0 {
            errors.push("agents.max_agents must be greater than 0".to_string());
        }
        
        if self.agents.cpu_limit_percent <= 0.0 || self.agents.cpu_limit_percent > 100.0 {
            errors.push("agents.cpu_limit_percent must be between 0 and 100".to_string());
        }
        
        // Validate memory config
        if !["sqlite", "json", "memory"].contains(&self.memory.backend.as_str()) {
            errors.push("memory.backend must be 'sqlite', 'json', or 'memory'".to_string());
        }
        
        if self.memory.max_entries == 0 {
            warnings.push("memory.max_entries is 0, memory will be unlimited".to_string());
        }
        
        // Validate neural config
        if self.neural.enabled && self.neural.model_path.is_empty() {
            errors.push("neural.model_path is required when neural processing is enabled".to_string());
        }
        
        // Validate swarm config
        if !["hierarchical", "mesh", "ring", "star"].contains(&self.swarm.default_topology.as_str()) {
            errors.push("swarm.default_topology must be 'hierarchical', 'mesh', 'ring', or 'star'".to_string());
        }
        
        // Validate GitHub config
        if self.github.enabled && self.github.token.is_none() {
            warnings.push("GitHub integration enabled but no token provided".to_string());
        }
        
        // Validate MCP config
        if self.mcp.enabled && self.mcp.servers.is_empty() {
            warnings.push("MCP enabled but no servers configured".to_string());
        }
        
        // Validate UI config
        if self.ui.web_ui_enabled && self.ui.web_ui_port == 0 {
            errors.push("ui.web_ui_port must be greater than 0 when web UI is enabled".to_string());
        }
        
        let valid = errors.is_empty();
        
        if !valid {
            error!("Configuration validation failed with {} errors", errors.len());
            for error in &errors {
                error!("Config error: {}", error);
            }
        }
        
        if !warnings.is_empty() {
            warn!("Configuration validation found {} warnings", warnings.len());
            for warning in &warnings {
                warn!("Config warning: {}", warning);
            }
        }
        
        Ok(ConfigValidation {
            valid,
            errors,
            warnings,
        })
    }
    
    pub async fn reset_to_defaults(&mut self) -> Result<()> {
        *self = Config::default();
        info!("Configuration reset to defaults");
        Ok(())
    }
    
    pub fn get_config_dir() -> Result<PathBuf> {
        Ok(directories::ProjectDirs::from("", "", "claude-flow")
            .context("Failed to determine config directory")?
            .config_dir()
            .to_path_buf())
    }
    
    pub fn get_data_dir() -> Result<PathBuf> {
        Ok(directories::ProjectDirs::from("", "", "claude-flow")
            .context("Failed to determine data directory")?
            .data_dir()
            .to_path_buf())
    }
    
    pub fn expand_path(&self, path: &str) -> PathBuf {
        if path.starts_with("~/") {
            if let Some(home) = directories::UserDirs::new() {
                return home.home_dir().join(&path[2..]);
            }
        }
        PathBuf::from(path)
    }
}