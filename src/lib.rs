// Claude Flow 2.0 - Core Library
// Complete Rust implementation with full feature parity

pub mod agents;
pub mod cli;
pub mod config;
pub mod coordination;
pub mod core;
pub mod enterprise;
#[cfg(feature = "github")]
pub mod github;
pub mod mcp;
pub mod memory;
pub mod neural;
pub mod swarm;
pub mod terminal;
pub mod ui;
pub mod utils;

// Re-export core types for external use
pub use agents::{Agent, AgentManager, AgentStatus, AgentType};
pub use config::Config;
pub use coordination::{Coordinator, CoordinationMessage, Task};
pub use core::{Core, CoreStatus};
pub use enterprise::{EnterpriseCoordinator, EnterpriseConfig, EnterpriseMetrics};
pub use memory::{MemoryEntry, MemoryManager, MemoryStats};
pub use swarm::{SwarmOrchestrator, SwarmStatus};

// Version information
pub const VERSION: &str = "2.0.0";
pub const BUILD_TIME: &str = "2025-01-01T00:00:00Z";
pub const GIT_HASH: &str = "development";

// Feature flags
#[cfg(feature = "neural")]
pub use neural::NeuralEngine;

#[cfg(feature = "github")]
pub use github::GitHubIntegration;

#[cfg(feature = "terminal-ui")]
pub use terminal::TerminalManager;

// Error types
pub use anyhow::{Error, Result};

// Prelude for common imports
pub mod prelude {
    pub use crate::{
        agents::{Agent, AgentManager, AgentType},
        config::Config,
        coordination::{Coordinator, Task},
        core::Core,
        enterprise::{EnterpriseCoordinator, EnterpriseConfig},
        memory::MemoryManager,
        swarm::SwarmOrchestrator,
        Result,
    };
}