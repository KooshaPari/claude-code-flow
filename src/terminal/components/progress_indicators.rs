//! Progress indicators component
//! Task progress and completion status displays

use anyhow::Result;
use tracing::info;

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
        Frame,
    },
};

#[derive(Debug, Clone)]
pub struct ProgressItem {
    pub id: String,
    pub name: String,
    pub progress: f64,
    pub status: String,
    pub eta: Option<std::time::Duration>,
}

pub struct ProgressIndicators {
    items: Vec<ProgressItem>,
    selected: usize,
}

impl ProgressIndicators {
    pub async fn new() -> Result<Self> {
        info!("Initializing progress indicators");
        
        let items = vec![
            ProgressItem {
                id: "task-001".to_string(),
                name: "Data Analysis".to_string(),
                progress: 0.75,
                status: "Processing".to_string(),
                eta: Some(std::time::Duration::from_secs(120)),
            },
            ProgressItem {
                id: "task-002".to_string(),
                name: "Model Training".to_string(),
                progress: 0.45,
                status: "Training".to_string(),
                eta: Some(std::time::Duration::from_secs(300)),
            },
            ProgressItem {
                id: "task-003".to_string(),
                name: "Report Generation".to_string(),
                progress: 1.0,
                status: "Completed".to_string(),
                eta: None,
            },
        ];
        
        Ok(Self {
            items,
            selected: 0,
        })
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints((0..self.items.len()).map(|_| Constraint::Length(4)).collect::<Vec<_>>())
            .split(area);

        for (i, item) in self.items.iter().enumerate() {
            if i < chunks.len() {
                self.draw_progress_item(f, chunks[i], item, i == self.selected);
            }
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_progress_item(&self, f: &mut Frame, area: Rect, item: &ProgressItem, selected: bool) {
        let title = if selected {
            format!("► {}", item.name)
        } else {
            format!("  {}", item.name)
        };

        let eta_text = if let Some(eta) = item.eta {
            format!(" (ETA: {}s)", eta.as_secs())
        } else {
            String::new()
        };

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_style(if selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Cyan)
                    }),
            )
            .gauge_style(Style::default().fg(
                if item.progress >= 1.0 { Color::Green } else { Color::Blue }
            ))
            .percent((item.progress * 100.0) as u16)
            .label(format!("{:.1}% - {}{}", item.progress * 100.0, item.status, eta_text));

        f.render_widget(gauge, area);
    }

    #[cfg(feature = "terminal-ui")]
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected < self.items.len().saturating_sub(1) {
                    self.selected += 1;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update progress values
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for item in &mut self.items {
            if item.progress < 1.0 {
                item.progress = (item.progress + rng.gen_range(0.01..0.05)).min(1.0);
                if item.progress >= 1.0 {
                    item.status = "Completed".to_string();
                    item.eta = None;
                }
            }
        }
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        info!("Refreshing progress indicators");
        Ok(())
    }
}