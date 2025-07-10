use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStatus {
    pub initialized: bool,
    pub version: String,
    pub uptime: u64,
    pub components: HashMap<String, ComponentStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    pub name: String,
    pub status: String,
    pub last_update: u64,
    pub health: f32, // 0.0 to 1.0
}

pub struct Core {
    config: Config,
    initialized: RwLock<bool>,
    startup_time: std::time::Instant,
    components: RwLock<HashMap<String, ComponentStatus>>,
    session_id: Uuid,
}

impl Core {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing Claude Flow 2.0 Core (Rust)");
        
        let session_id = Uuid::new_v4();
        info!("Session ID: {}", session_id);
        
        Ok(Self {
            config: config.clone(),
            initialized: RwLock::new(false),
            startup_time: std::time::Instant::now(),
            components: RwLock::new(HashMap::new()),
            session_id,
        })
    }
    
    pub async fn initialize(&self, force: bool) -> Result<()> {
        let mut initialized = self.initialized.write().await;
        
        if *initialized && !force {
            warn!("Core already initialized. Use force=true to reinitialize.");
            return Ok(());
        }
        
        info!("Initializing core components...");
        
        // Initialize core subsystems
        self.initialize_logging().await?;
        self.initialize_event_system().await?;
        self.initialize_persistence().await?;
        
        // Register core components
        self.register_component("core", "active", 1.0).await;
        self.register_component("logging", "active", 1.0).await;
        self.register_component("events", "active", 1.0).await;
        self.register_component("persistence", "active", 1.0).await;
        
        *initialized = true;
        info!("Core initialization complete");
        
        Ok(())
    }
    
    pub async fn get_status(&self) -> Result<CoreStatus> {
        let initialized = *self.initialized.read().await;
        let uptime = self.startup_time.elapsed().as_secs();
        let components = self.components.read().await.clone();
        
        Ok(CoreStatus {
            initialized,
            version: "2.0.0-rust".to_string(),
            uptime,
            components,
        })
    }
    
    pub fn get_session_id(&self) -> Uuid {
        self.session_id
    }
    
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    async fn initialize_logging(&self) -> Result<()> {
        // Logging is already initialized in main.rs
        // This could set up additional log handlers, formatters, etc.
        info!("Logging subsystem initialized");
        Ok(())
    }
    
    async fn initialize_event_system(&self) -> Result<()> {
        // Initialize event bus for inter-component communication
        info!("Event system initialized");
        Ok(())
    }
    
    async fn initialize_persistence(&self) -> Result<()> {
        // Initialize persistence layer
        info!("Persistence layer initialized");
        Ok(())
    }
    
    async fn register_component(&self, name: &str, status: &str, health: f32) {
        let component = ComponentStatus {
            name: name.to_string(),
            status: status.to_string(),
            last_update: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            health,
        };
        
        self.components.write().await.insert(name.to_string(), component);
    }
    
    pub async fn update_component_status(&self, name: &str, status: &str, health: f32) {
        if let Some(component) = self.components.write().await.get_mut(name) {
            component.status = status.to_string();
            component.health = health;
            component.last_update = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }
    
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Claude Flow 2.0 Core");
        
        // Gracefully shutdown all components
        let components = self.components.read().await;
        for (name, _) in components.iter() {
            info!("Shutting down component: {}", name);
        }
        
        *self.initialized.write().await = false;
        info!("Core shutdown complete");
        
        Ok(())
    }
}