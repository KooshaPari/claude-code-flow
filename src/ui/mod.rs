use anyhow::Result;
use tracing::info;

use crate::config::Config;

pub struct UiManager {
    _config: crate::config::UiConfig,
}

impl UiManager {
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Initializing UI manager");
        Ok(Self {
            _config: config.ui.clone(),
        })
    }
}