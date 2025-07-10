//! Log viewer component
//! Structured log browsing with filtering

use anyhow::Result;
use tracing::info;

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, List, ListItem, Paragraph},
        Frame,
    },
};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    #[cfg(feature = "terminal-ui")]
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Error => Color::Red,
            LogLevel::Warn => Color::Yellow,
            LogLevel::Info => Color::Green,
            LogLevel::Debug => Color::Cyan,
            LogLevel::Trace => Color::Gray,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN ",
            LogLevel::Info => "INFO ",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
}

pub struct LogViewer {
    logs: Vec<LogEntry>,
    filtered_logs: Vec<usize>,
    scroll_offset: usize,
    filter_level: Option<LogLevel>,
    filter_component: Option<String>,
    auto_scroll: bool,
}

impl LogViewer {
    pub async fn new() -> Result<Self> {
        info!("Initializing log viewer");
        
        let logs = vec![
            LogEntry {
                timestamp: chrono::Utc::now() - chrono::Duration::seconds(120),
                level: LogLevel::Info,
                component: "agent-manager".to_string(),
                message: "Agent spawned successfully: agent-001".to_string(),
            },
            LogEntry {
                timestamp: chrono::Utc::now() - chrono::Duration::seconds(90),
                level: LogLevel::Debug,
                component: "swarm-coordinator".to_string(),
                message: "Task assigned to agent-001: data-analysis".to_string(),
            },
            LogEntry {
                timestamp: chrono::Utc::now() - chrono::Duration::seconds(60),
                level: LogLevel::Warn,
                component: "memory-manager".to_string(),
                message: "Cache hit rate below threshold: 85%".to_string(),
            },
            LogEntry {
                timestamp: chrono::Utc::now() - chrono::Duration::seconds(30),
                level: LogLevel::Error,
                component: "neural-network".to_string(),
                message: "Training failed: insufficient data".to_string(),
            },
            LogEntry {
                timestamp: chrono::Utc::now() - chrono::Duration::seconds(10),
                level: LogLevel::Info,
                component: "task-executor".to_string(),
                message: "Task completed successfully: task-001".to_string(),
            },
        ];

        let filtered_logs: Vec<usize> = (0..logs.len()).collect();

        Ok(Self {
            logs,
            filtered_logs,
            scroll_offset: 0,
            filter_level: None,
            filter_component: None,
            auto_scroll: true,
        })
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        self.draw_log_list(f, chunks[0]);
        self.draw_status_bar(f, chunks[1]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_log_list(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.filtered_logs
            .iter()
            .skip(self.scroll_offset)
            .take(area.height as usize - 2) // Account for borders
            .filter_map(|&idx| self.logs.get(idx))
            .map(|entry| {
                ListItem::new(Line::from(vec![
                    Span::styled(
                        entry.timestamp.format("%H:%M:%S").to_string(),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        entry.level.as_str(),
                        Style::default().fg(entry.level.color()).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!("[{}]", entry.component),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        entry.message.clone(),
                        Style::default().fg(Color::White),
                    ),
                ]))
            })
            .collect();

        let log_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(
                        "Logs ({}/{}) - Auto-scroll: {}",
                        self.filtered_logs.len(),
                        self.logs.len(),
                        if self.auto_scroll { "ON" } else { "OFF" }
                    ))
                    .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            );

        f.render_widget(log_list, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        let filter_info = match (&self.filter_level, &self.filter_component) {
            (Some(level), Some(component)) => format!("Level: {} | Component: {}", level.as_str(), component),
            (Some(level), None) => format!("Level: {}", level.as_str()),
            (None, Some(component)) => format!("Component: {}", component),
            (None, None) => "No filters".to_string(),
        };

        let status_text = format!(
            "Filters: {} | F: Filter | C: Clear | S: Save | A: Auto-scroll toggle",
            filter_info
        );

        let status = Paragraph::new(status_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Controls")
                    .title_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(status, area);
    }

    #[cfg(feature = "terminal-ui")]
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                    self.auto_scroll = false;
                }
            }
            KeyCode::Down => {
                if self.scroll_offset + 1 < self.filtered_logs.len() {
                    self.scroll_offset += 1;
                    self.auto_scroll = false;
                }
            }
            KeyCode::PageUp => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
                self.auto_scroll = false;
            }
            KeyCode::PageDown => {
                self.scroll_offset = (self.scroll_offset + 10).min(self.filtered_logs.len().saturating_sub(1));
                self.auto_scroll = false;
            }
            KeyCode::Home => {
                self.scroll_offset = 0;
                self.auto_scroll = false;
            }
            KeyCode::End => {
                self.scroll_offset = self.filtered_logs.len().saturating_sub(1);
                self.auto_scroll = true;
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                // Toggle through log levels
                self.filter_level = match self.filter_level {
                    None => Some(LogLevel::Error),
                    Some(LogLevel::Error) => Some(LogLevel::Warn),
                    Some(LogLevel::Warn) => Some(LogLevel::Info),
                    Some(LogLevel::Info) => Some(LogLevel::Debug),
                    Some(LogLevel::Debug) => Some(LogLevel::Trace),
                    Some(LogLevel::Trace) => None,
                };
                self.apply_filters();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.filter_level = None;
                self.filter_component = None;
                self.apply_filters();
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.auto_scroll = !self.auto_scroll;
                if self.auto_scroll {
                    self.scroll_offset = self.filtered_logs.len().saturating_sub(1);
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // Save logs (placeholder)
                info!("Saving logs...");
            }
            _ => {}
        }
        Ok(())
    }

    fn apply_filters(&mut self) {
        self.filtered_logs = self.logs
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                let level_match = self.filter_level.as_ref()
                    .map(|level| &entry.level == level)
                    .unwrap_or(true);
                
                let component_match = self.filter_component.as_ref()
                    .map(|comp| entry.component.contains(comp))
                    .unwrap_or(true);
                
                level_match && component_match
            })
            .map(|(idx, _)| idx)
            .collect();
        
        self.scroll_offset = 0;
    }

    pub async fn update(&mut self) -> Result<()> {
        // Simulate new log entries
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if rng.gen_bool(0.3) {
            let levels = [LogLevel::Info, LogLevel::Debug, LogLevel::Warn];
            let components = ["agent-manager", "swarm-coordinator", "memory-manager", "task-executor"];
            let messages = [
                "Operation completed successfully",
                "Processing task",
                "Cache miss detected",
                "Network connection established",
                "Resource usage normal",
            ];

            let new_entry = LogEntry {
                timestamp: chrono::Utc::now(),
                level: levels[rng.gen_range(0..levels.len())].clone(),
                component: components[rng.gen_range(0..components.len())].to_string(),
                message: messages[rng.gen_range(0..messages.len())].to_string(),
            };

            self.logs.push(new_entry);
            
            // Keep only last 1000 entries
            if self.logs.len() > 1000 {
                self.logs.remove(0);
            }
            
            self.apply_filters();
            
            if self.auto_scroll {
                self.scroll_offset = self.filtered_logs.len().saturating_sub(1);
            }
        }
        
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        info!("Refreshing log viewer");
        self.apply_filters();
        Ok(())
    }
}