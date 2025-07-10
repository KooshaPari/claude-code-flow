use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber;

mod cli;
mod core;
mod agents;
mod coordination;
mod memory;
mod mcp;
mod swarm;
mod terminal;
mod config;
mod neural;
#[cfg(feature = "github")]
mod github;
mod ui;
mod utils;

use cli::CliApp;

#[derive(Parser)]
#[command(name = "claude-flow")]
#[command(about = "Claude Flow 2.0 - AI Orchestration Platform (Rust Implementation)")]
#[command(version = "2.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Claude Flow configuration
    Init {
        /// Force initialization even if config exists
        #[arg(long)]
        force: bool,
    },
    /// Show system status
    Status,
    /// Show version information  
    Version,
    /// Run interactive mode
    Interactive,
    /// Memory operations
    Memory {
        #[command(subcommand)]
        action: MemoryAction,
    },
    /// Agent operations
    Agents {
        #[command(subcommand)]
        action: AgentAction,
    },
    /// Swarm operations
    Swarm {
        #[command(subcommand)]
        action: SwarmAction,
    },
}

#[derive(Subcommand)]
enum MemoryAction {
    Store { key: String, value: String, namespace: Option<String> },
    Retrieve { key: String, namespace: Option<String> },
    List { namespace: Option<String> },
}

#[derive(Subcommand)]
enum AgentAction {
    Spawn { count: u32, strategy: Option<String> },
    List,
    Status,
}

#[derive(Subcommand)]
enum SwarmAction {
    Init { topology: String, max_agents: u32 },
    Status,
    Monitor,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt().with_max_level(level).init();
    
    info!("Claude Flow 2.0 (Rust) starting...");
    
    let mut app = CliApp::new(cli.config).await?;
    
    match cli.command {
        Some(command) => {
            match command {
                Commands::Init { force } => {
                    app.init(force).await?;
                },
                Commands::Status => {
                    app.status().await?;
                },
                Commands::Version => {
                    app.version().await?;
                },
                Commands::Interactive => {
                    app.run_interactive().await?;
                },
                Commands::Memory { action } => {
                    match action {
                        MemoryAction::Store { key, value, namespace } => {
                            println!("Storing: {}={} in {:?}", key, value, namespace);
                        },
                        MemoryAction::Retrieve { key, namespace } => {
                            println!("Retrieving: {} from {:?}", key, namespace);
                        },
                        MemoryAction::List { namespace } => {
                            println!("Listing entries in {:?}", namespace);
                        },
                    }
                },
                Commands::Agents { action } => {
                    match action {
                        AgentAction::Spawn { count, strategy } => {
                            println!("Spawning {} agents with strategy {:?}", count, strategy);
                        },
                        AgentAction::List => {
                            println!("Listing agents");
                        },
                        AgentAction::Status => {
                            println!("Agent status");
                        },
                    }
                },
                Commands::Swarm { action } => {
                    match action {
                        SwarmAction::Init { topology, max_agents } => {
                            println!("Initializing swarm: {} with {} agents", topology, max_agents);
                        },
                        SwarmAction::Status => {
                            println!("Swarm status");
                        },
                        SwarmAction::Monitor => {
                            println!("Monitoring swarm");
                        },
                    }
                },
            }
        },
        None => {
            // Default behavior - show help or run interactive mode
            println!("🌊 Claude Flow 2.0 - AI Orchestration Platform");
            println!("Run with --help for usage information");
            println!("Run 'claude-flow interactive' for interactive mode");
        }
    }
    
    Ok(())
}