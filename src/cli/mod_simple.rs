use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

use crate::config::Config;
use crate::core::Core;
use crate::agents::AgentManager;
use crate::coordination::Coordinator;
use crate::memory::MemoryManager;
use crate::neural::NeuralEngine;
use crate::swarm::SwarmOrchestrator;
use crate::github::GitHubIntegration;
use crate::mcp::McpClient;
use crate::terminal::TerminalManager;

pub mod interactive;
pub mod commands;
pub mod commands_simple;
pub mod ui;

pub struct CliApp {
    config: Config,
    core: Core,
    agent_manager: AgentManager,
    coordinator: Coordinator,
    memory_manager: MemoryManager,
    neural_engine: NeuralEngine,
    swarm_orchestrator: SwarmOrchestrator,
    github_integration: GitHubIntegration,
    mcp_client: McpClient,
    terminal_manager: TerminalManager,
}

impl CliApp {
    pub async fn new(config_path: Option<String>) -> Result<Self> {
        let config_file = config_path
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                dirs::config_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("claude-flow")
                    .join("config.toml")
            });
            
        let config = Config::load(&config_file).await?;
        let core = Core::new(&config).await?;
        
        let agent_manager = AgentManager::new(&config).await?;
        let coordinator = Coordinator::new(&config).await?;
        let memory_manager = MemoryManager::new(&config).await?;
        let neural_engine = NeuralEngine::new(&config).await?;
        let swarm_orchestrator = SwarmOrchestrator::new(&config).await?;
        let github_integration = GitHubIntegration::new(&config).await?;
        let mcp_client = McpClient::new(&config).await?;
        let terminal_manager = TerminalManager::new().await?;

        Ok(Self {
            config,
            core,
            agent_manager,
            coordinator,
            memory_manager,
            neural_engine,
            swarm_orchestrator,
            github_integration,
            mcp_client,
            terminal_manager,
        })
    }
    
    pub async fn run_interactive(&self) -> Result<()> {
        let interactive = interactive::InteractiveCli::new(self.config.clone());
        interactive.run().await
    }
    
    pub async fn init(&mut self, force: bool) -> Result<()> {
        commands_simple::CommandHandler::init(force).await
    }
    
    pub async fn status(&self) -> Result<()> {
        commands_simple::CommandHandler::status().await
    }
    
    pub async fn version(&self) -> Result<()> {
        commands_simple::CommandHandler::version().await
    }
}