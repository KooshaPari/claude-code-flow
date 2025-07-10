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
        /// Enable hive-mind features
        #[arg(long)]
        hive_mind: bool,
        /// Enable neural enhancement
        #[arg(long)]
        neural_enhanced: bool,
    },
    /// Hive-mind coordination commands
    #[command(subcommand)]
    HiveMind(HiveMindCommands),
    /// Memory management commands
    #[command(subcommand)]
    Memory(MemoryCommands),
    /// Neural network commands
    #[command(subcommand)]
    Neural(NeuralCommands),
    /// Swarm orchestration commands
    #[command(subcommand)]
    Swarm(SwarmCommands),
    /// GitHub integration commands
    #[command(subcommand)]
    Github(GithubCommands),
    /// MCP (Model Context Protocol) commands
    #[command(subcommand)]
    Mcp(McpCommands),
    /// Configuration management
    #[command(subcommand)]
    Config(ConfigCommands),
    /// Start interactive session
    Start,
    /// Start REPL interface
    Repl,
    /// Start dashboard view
    Dashboard,
    /// Show status information
    Status,
}

#[derive(Subcommand)]
enum HiveMindCommands {
    /// Interactive hive-mind setup wizard
    Wizard,
    /// Spawn intelligent swarm for task execution
    Spawn {
        /// Task description
        task: String,
        /// Number of agents to spawn
        #[arg(long, default_value = "5")]
        agents: u32,
        /// Coordination strategy
        #[arg(long, default_value = "hierarchical")]
        strategy: String,
        /// Enable Claude integration
        #[arg(long)]
        claude: bool,
    },
    /// Check hive-mind status
    Status {
        /// Enable real-time monitoring
        #[arg(long)]
        real_time: bool,
    },
    /// Optimize hive-mind performance
    Optimize,
}

#[derive(Subcommand)]
enum MemoryCommands {
    /// Store key-value pair in memory
    Store {
        /// Memory key
        key: String,
        /// Memory value
        value: String,
        /// Memory namespace
        #[arg(long, default_value = "default")]
        namespace: String,
    },
    /// Query memory entries
    Query {
        /// Search pattern
        pattern: String,
        /// Memory namespace
        #[arg(long)]
        namespace: Option<String>,
    },
    /// Show memory statistics
    Stats,
    /// List all memory namespaces
    List,
    /// Export memory to file
    Export {
        /// Output file path
        file: String,
        /// Memory namespace
        #[arg(long)]
        namespace: Option<String>,
    },
    /// Import memory from file
    Import {
        /// Input file path
        file: String,
    },
}

#[derive(Subcommand)]
enum NeuralCommands {
    /// Train neural patterns
    Train {
        /// Pattern type
        #[arg(long)]
        pattern: String,
        /// Training data file
        #[arg(long)]
        data: Option<String>,
        /// Number of epochs
        #[arg(long, default_value = "100")]
        epochs: u32,
    },
    /// Make predictions using neural models
    Predict {
        /// Model name
        #[arg(long)]
        model: String,
        /// Input data
        #[arg(long)]
        input: String,
    },
    /// Analyze cognitive behavior
    Analyze {
        /// Behavior type
        #[arg(long)]
        behavior: String,
    },
}

#[derive(Subcommand)]
enum SwarmCommands {
    /// Initialize swarm coordination
    Init {
        /// Topology type
        #[arg(long, default_value = "hierarchical")]
        topology: String,
        /// Maximum number of agents
        #[arg(long, default_value = "8")]
        max_agents: u32,
    },
    /// Monitor swarm status
    Monitor {
        /// Enable dashboard view
        #[arg(long)]
        dashboard: bool,
        /// Enable real-time updates
        #[arg(long)]
        real_time: bool,
    },
    /// Execute task with swarm coordination
    Execute {
        /// Task description
        task: String,
        /// Execution strategy
        #[arg(long, default_value = "parallel")]
        strategy: String,
    },
}

#[derive(Subcommand)]
enum GithubCommands {
    /// GitHub coordinator analysis
    GhCoordinator {
        /// Analysis type
        #[arg(long, default_value = "security")]
        analysis_type: String,
    },
    /// Pull request management
    PrManager {
        /// Enable multi-reviewer mode
        #[arg(long)]
        multi_reviewer: bool,
        /// Enable AI-powered reviews
        #[arg(long)]
        ai_powered: bool,
    },
    /// Repository architecture optimization
    RepoArchitect {
        /// Enable structure analysis
        #[arg(long)]
        structure_analysis: bool,
    },
}

#[derive(Subcommand)]
enum McpCommands {
    /// Setup MCP servers
    Setup {
        /// Enable auto-permissions
        #[arg(long)]
        auto_permissions: bool,
        /// Enable all 87 tools
        #[arg(long)]
        tools_87: bool,
    },
    /// List available MCP tools
    List,
    /// Test MCP connectivity
    Test,
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Validate configuration
    Validate,
    /// Reset configuration to defaults
    Reset,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
    
    info!("Claude Flow 2.0 (Rust) starting...");
    
    // Initialize CLI app
    let mut app = CliApp::new(cli.config).await?;
    
    // Execute command
    match cli.command {
        Some(Commands::Init { force, hive_mind, neural_enhanced }) => {
            app.init(force, hive_mind, neural_enhanced).await?;
        }
        Some(Commands::HiveMind(cmd)) => {
            app.handle_hive_mind_command(cmd).await?;
        }
        Some(Commands::Memory(cmd)) => {
            app.handle_memory_command(cmd).await?;
        }
        Some(Commands::Neural(cmd)) => {
            app.handle_neural_command(cmd).await?;
        }
        Some(Commands::Swarm(cmd)) => {
            app.handle_swarm_command(cmd).await?;
        }
        Some(Commands::Github(cmd)) => {
            app.handle_github_command(cmd).await?;
        }
        Some(Commands::Mcp(cmd)) => {
            app.handle_mcp_command(cmd).await?;
        }
        Some(Commands::Config(cmd)) => {
            app.handle_config_command(cmd).await?;
        }
        Some(Commands::Start) => {
            app.start_interactive().await?;
        }
        Some(Commands::Repl) => {
            app.start_repl().await?;
        }
        Some(Commands::Dashboard) => {
            app.start_dashboard().await?;
        }
        Some(Commands::Status) => {
            app.show_status().await?;
        }
        None => {
            app.show_help().await?;
        }
    }
    
    Ok(())
}
