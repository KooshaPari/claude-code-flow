use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub network_io: u64,
    pub disk_io: u64,
}

pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

pub fn expand_home_path(path: &str) -> Result<std::path::PathBuf> {
    if path.starts_with("~/") {
        if let Some(home) = directories::UserDirs::new() {
            Ok(home.home_dir().join(&path[2..]))
        } else {
            Ok(std::path::PathBuf::from(path))
        }
    } else {
        Ok(std::path::PathBuf::from(path))
    }
}

pub async fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        tokio::fs::create_dir_all(path).await?;
    }
    Ok(())
}

pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}