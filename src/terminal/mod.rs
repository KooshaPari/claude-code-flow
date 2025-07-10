//! Terminal UI system for Claude Flow
//! Provides interactive terminal interface with real-time monitoring

pub mod repl;
pub mod dashboard;
pub mod components;
pub mod events;
pub mod themes;
pub mod app;

use anyhow::Result;
use tracing::info;

use crate::config::Config;

pub use app::TerminalApp;
pub use repl::InteractiveRepl;
pub use dashboard::RealTimeDashboard;

/// Main terminal manager for orchestrating all UI components
pub struct TerminalManager {
    config: crate::config::UiConfig,
    app: Option<TerminalApp>,
}

impl TerminalManager {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing terminal manager");
        Ok(Self {
            config: config.ui.clone(),
            app: None,
        })
    }

    /// Start the interactive terminal application
    pub async fn start_interactive(&mut self) -> Result<()> {
        let mut app = TerminalApp::new(&self.config).await?;
        app.run().await?;
        self.app = Some(app);
        Ok(())
    }

    /// Start the REPL interface
    pub async fn start_repl(&mut self) -> Result<()> {
        let repl = InteractiveRepl::new(&self.config).await?;
        repl.run().await
    }

    /// Start the dashboard view
    pub async fn start_dashboard(&mut self) -> Result<()> {
        let dashboard = RealTimeDashboard::new(&self.config).await?;
        dashboard.run().await
    }

    /// Get current terminal dimensions
    pub fn get_dimensions(&self) -> Result<(u16, u16)> {
        #[cfg(feature = "terminal-ui")]
        {
            use crossterm::terminal;
            Ok(terminal::size()?)
        }
        #[cfg(not(feature = "terminal-ui"))]
        {
            Ok((80, 24))
        }
    }
}