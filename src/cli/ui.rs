// CLI UI utilities for Claude Flow 2.0
use anyhow::Result;

pub fn show_banner() {
    println!("🌊 Claude Flow 2.0 - AI Orchestration Platform");
    println!("================================================");
}

pub fn show_progress(message: &str) {
    println!("⏳ {}", message);
}

pub fn show_success(message: &str) {
    println!("✅ {}", message);
}

pub fn show_error(message: &str) {
    eprintln!("❌ {}", message);
}

pub fn show_warning(message: &str) {
    println!("⚠️  {}", message);
}

pub async fn confirm(message: &str) -> Result<bool> {
    print!("{} [y/N]: ", message);
    use std::io::{self, Write};
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_lowercase() == "y")
}