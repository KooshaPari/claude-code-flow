use anyhow::Result;
use tracing::info;

use crate::config::Config;

pub struct TerminalManager {
    _config: crate::config::UiConfig,
}

impl TerminalManager {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing terminal manager");
        Ok(Self {
            _config: config.ui.clone(),
        })
    }
}