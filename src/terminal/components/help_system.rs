//! Help system component
//! Contextual help and documentation

use anyhow::Result;
use tracing::info;

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
        text::{Line, Span},
        Frame,
    },
};

pub struct HelpSystem {
    current_topic: String,
}

impl HelpSystem {
    pub async fn new() -> Result<Self> {
        info!("Initializing help system");
        Ok(Self {
            current_topic: "overview".to_string(),
        })
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let help_text = self.get_help_content(&self.current_topic);
        
        let help_paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help System")
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default().fg(Color::White))
            .wrap(ratatui::widgets::Wrap { trim: true });

        f.render_widget(help_paragraph, area);
    }

    fn get_help_content(&self, topic: &str) -> String {
        match topic {
            "overview" => {
                "Claude Flow Terminal UI Help\\n\
                 ═══════════════════════════\\n\\n\
                 Welcome to Claude Flow's Terminal User Interface!\\n\\n\
                 Navigation:\\n\
                 • Tab/Shift+Tab - Switch between modes\\n\
                 • 1-9 - Jump to mode by number\\n\
                 • F1/? - Toggle help\\n\
                 • q/Ctrl+C - Quit\\n\\n\
                 Available Modes:\\n\
                 1. Dashboard - Real-time system overview\\n\
                 2. REPL - Interactive command line\\n\
                 3. Agents - Agent monitoring and management\\n\
                 4. Memory - Memory browser and search\\n\
                 5. Swarm - Swarm topology visualization\\n\
                 6. Progress - Task progress tracking\\n\
                 7. Logs - Log viewer with filtering\\n\
                 8. Config - Configuration editor\\n\
                 9. Help - This help system\\n\
                 10. Performance - Performance monitoring\\n\\n\
                 Each mode has its own keyboard shortcuts - press F1 while in a mode for specific help.".to_string()
            }
            _ => "Help topic not found".to_string(),
        }
    }

    #[cfg(feature = "terminal-ui")]
    pub async fn handle_key(&mut self, _key: KeyEvent) -> Result<()> {
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        info!("Refreshing help system");
        Ok(())
    }
}