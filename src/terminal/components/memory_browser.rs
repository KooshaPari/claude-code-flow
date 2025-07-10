//! Memory browser component
//! Interactive memory exploration and management

use anyhow::Result;
use std::collections::HashMap;
use tracing::info;

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect, Alignment},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{
            Block, Borders, List, ListItem, Table, Row, Cell,
            Paragraph, Clear, Tabs,
        },
        Frame,
    },
};

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
    pub namespace: String,
    pub size: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum ViewMode {
    Browser,
    Details,
    Search,
}

pub struct MemoryBrowser {
    entries: Vec<MemoryEntry>,
    namespaces: HashMap<String, usize>,
    selected_entry: Option<usize>,
    view_mode: ViewMode,
    tab_index: usize,
    search_query: String,
    filtered_entries: Vec<usize>,
    current_namespace: Option<String>,
    show_search_dialog: bool,
}

impl MemoryBrowser {
    pub async fn new() -> Result<Self> {
        info!("Initializing memory browser");
        
        // Generate mock memory entries
        let entries = vec![
            MemoryEntry {
                key: "swarm-config".to_string(),
                value: r#"{"topology": "hierarchical", "max_agents": 8}"#.to_string(),
                namespace: "system".to_string(),
                size: 45,
                created_at: chrono::Utc::now() - chrono::Duration::hours(2),
                accessed_at: chrono::Utc::now() - chrono::Duration::minutes(5),
                access_count: 15,
                tags: vec!["config".to_string(), "swarm".to_string()],
            },
            MemoryEntry {
                key: "agent-001-state".to_string(),
                value: r#"{"status": "active", "tasks": 3, "cpu": 0.25}"#.to_string(),
                namespace: "agents".to_string(),
                size: 52,
                created_at: chrono::Utc::now() - chrono::Duration::hours(1),
                accessed_at: chrono::Utc::now() - chrono::Duration::minutes(1),
                access_count: 42,
                tags: vec!["agent".to_string(), "state".to_string()],
            },
            MemoryEntry {
                key: "task-results-analysis".to_string(),
                value: "Analysis completed with 95% accuracy. Found 12 key insights.".to_string(),
                namespace: "tasks".to_string(),
                size: 67,
                created_at: chrono::Utc::now() - chrono::Duration::minutes(30),
                accessed_at: chrono::Utc::now() - chrono::Duration::seconds(30),
                access_count: 8,
                tags: vec!["task".to_string(), "results".to_string(), "analysis".to_string()],
            },
        ];

        let mut namespaces = HashMap::new();
        for entry in &entries {
            *namespaces.entry(entry.namespace.clone()).or_insert(0) += 1;
        }

        Ok(Self {
            entries,
            namespaces,
            selected_entry: Some(0),
            view_mode: ViewMode::Browser,
            tab_index: 0,
            search_query: String::new(),
            filtered_entries: (0..3).collect(),
            current_namespace: None,
            show_search_dialog: false,
        })
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        if self.show_search_dialog {
            self.draw_search_dialog(f, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Status
            ])
            .split(area);

        self.draw_tabs(f, chunks[0]);
        
        match self.view_mode {
            ViewMode::Browser => self.draw_browser(f, chunks[1]),
            ViewMode::Details => self.draw_details(f, chunks[1]),
            ViewMode::Search => self.draw_search_results(f, chunks[1]),
        }
        
        self.draw_status_bar(f, chunks[2]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        let titles = vec!["Browser", "Details", "Search"];
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory Browser")
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .select(self.tab_index);
        
        f.render_widget(tabs, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_browser(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        self.draw_namespaces(f, chunks[0]);
        self.draw_entries_table(f, chunks[1]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_namespaces(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.namespaces
            .iter()
            .map(|(namespace, count)| {
                let is_selected = self.current_namespace.as_ref() == Some(namespace);
                let style = if is_selected {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default().fg(Color::Cyan)
                };
                
                ListItem::new(Line::from(vec![
                    Span::styled(format!("{:<15}", namespace), style),
                    Span::styled(format!("({})", count), Style::default().fg(Color::Gray)),
                ]))
            })
            .collect();

        let namespace_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Namespaces")
                    .title_style(Style::default().fg(Color::Magenta)),
            );

        f.render_widget(namespace_list, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_entries_table(&self, f: &mut Frame, area: Rect) {
        let header = Row::new(vec![
            Cell::from("Key").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Namespace").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Size").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Access Count").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Last Access").style(Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let entries_to_show: Vec<&MemoryEntry> = if let Some(ns) = &self.current_namespace {
            self.entries.iter().filter(|e| &e.namespace == ns).collect()
        } else {
            self.entries.iter().collect()
        };

        let rows: Vec<Row> = entries_to_show
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let style = if Some(i) == self.selected_entry {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(entry.key.clone()),
                    Cell::from(entry.namespace.clone()),
                    Cell::from(format!("{} B", entry.size)),
                    Cell::from(entry.access_count.to_string()),
                    Cell::from(format_time_ago(entry.accessed_at)),
                ]).style(style)
            })
            .collect();

        let table = Table::new(rows, [
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(15),
        ])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Memory Entries ({})", entries_to_show.len()))
                .title_style(Style::default().fg(Color::Green)),
        );

        f.render_widget(table, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_details(&self, f: &mut Frame, area: Rect) {
        if let Some(idx) = self.selected_entry {
            let entries_to_show: Vec<&MemoryEntry> = if let Some(ns) = &self.current_namespace {
                self.entries.iter().filter(|e| &e.namespace == ns).collect()
            } else {
                self.entries.iter().collect()
            };

            if let Some(entry) = entries_to_show.get(idx) {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(8),  // Metadata
                        Constraint::Min(0),     // Content
                    ])
                    .split(area);

                // Metadata
                let metadata_text = format!(
                    "Key: {}\\n\
                     Namespace: {}\\n\
                     Size: {} bytes\\n\
                     Created: {}\\n\
                     Last Access: {}\\n\
                     Access Count: {}\\n\
                     Tags: {}",
                    entry.key,
                    entry.namespace,
                    entry.size,
                    entry.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
                    entry.accessed_at.format("%Y-%m-%d %H:%M:%S UTC"),
                    entry.access_count,
                    entry.tags.join(", ")
                );

                let metadata = Paragraph::new(metadata_text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Metadata")
                            .title_style(Style::default().fg(Color::Cyan)),
                    )
                    .style(Style::default().fg(Color::White));

                f.render_widget(metadata, chunks[0]);

                // Content
                let content = Paragraph::new(entry.value.clone())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Content")
                            .title_style(Style::default().fg(Color::Yellow)),
                    )
                    .style(Style::default().fg(Color::White))
                    .wrap(ratatui::widgets::Wrap { trim: true });

                f.render_widget(content, chunks[1]);
            }
        } else {
            let no_selection = Paragraph::new("No entry selected")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Details"),
                )
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));
            f.render_widget(no_selection, area);
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_search_results(&self, f: &mut Frame, area: Rect) {
        let search_results: Vec<&MemoryEntry> = self.filtered_entries
            .iter()
            .filter_map(|&idx| self.entries.get(idx))
            .collect();

        let header = Row::new(vec![
            Cell::from("Key").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Value Preview").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Namespace").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Tags").style(Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let rows: Vec<Row> = search_results
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let style = if Some(i) == self.selected_entry {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                let preview = if entry.value.len() > 30 {
                    format!("{}...", &entry.value[..27])
                } else {
                    entry.value.clone()
                };

                Row::new(vec![
                    Cell::from(entry.key.clone()),
                    Cell::from(preview),
                    Cell::from(entry.namespace.clone()),
                    Cell::from(entry.tags.join(", ")),
                ]).style(style)
            })
            .collect();

        let table = Table::new(rows, [
            Constraint::Min(20),
            Constraint::Min(30),
            Constraint::Length(12),
            Constraint::Min(15),
        ])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Search Results ({}) - Query: '{}'", search_results.len(), self.search_query))
                .title_style(Style::default().fg(Color::Magenta)),
        );

        f.render_widget(table, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_search_dialog(&self, f: &mut Frame, area: Rect) {
        let popup_area = self.centered_rect(60, 20, area);
        
        let dialog_text = format!(
            "Search Memory\\n\
             ─────────────\\n\
             \\n\
             Query: {}\\n\
             \\n\
             Search in keys, values, namespaces, and tags\\n\
             Press Enter to search, Esc to cancel",
            if self.search_query.is_empty() { "_" } else { &self.search_query }
        );

        let dialog = Paragraph::new(dialog_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Search")
                    .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);

        f.render_widget(Clear, popup_area);
        f.render_widget(dialog, popup_area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        let status_text = format!(
            "Total: {} entries | Namespaces: {} | Selected: {} | /: Search | Tab: Switch View",
            self.entries.len(),
            self.namespaces.len(),
            if let Some(idx) = self.selected_entry {
                format!("{}/{}", idx + 1, self.entries.len())
            } else {
                "None".to_string()
            }
        );

        let status = Paragraph::new(status_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Status")
                    .title_style(Style::default().fg(Color::Green)),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        f.render_widget(status, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    #[cfg(feature = "terminal-ui")]
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if self.show_search_dialog {
            match key.code {
                KeyCode::Esc => {
                    self.show_search_dialog = false;
                    self.search_query.clear();
                }
                KeyCode::Enter => {
                    self.perform_search();
                    self.show_search_dialog = false;
                    self.view_mode = ViewMode::Search;
                    self.tab_index = 2;
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                }
                _ => {}
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Tab => {
                self.tab_index = (self.tab_index + 1) % 3;
                self.view_mode = match self.tab_index {
                    0 => ViewMode::Browser,
                    1 => ViewMode::Details,
                    2 => ViewMode::Search,
                    _ => ViewMode::Browser,
                };
            }
            KeyCode::Up => {
                let len = if let Some(ns) = &self.current_namespace {
                    self.entries.iter().filter(|e| &e.namespace == ns).count()
                } else {
                    self.entries.len()
                };
                
                if len > 0 {
                    self.selected_entry = Some(match self.selected_entry {
                        Some(idx) if idx > 0 => idx - 1,
                        Some(_) => len - 1,
                        None => 0,
                    });
                }
            }
            KeyCode::Down => {
                let len = if let Some(ns) = &self.current_namespace {
                    self.entries.iter().filter(|e| &e.namespace == ns).count()
                } else {
                    self.entries.len()
                };
                
                if len > 0 {
                    self.selected_entry = Some(match self.selected_entry {
                        Some(idx) if idx < len - 1 => idx + 1,
                        Some(_) => 0,
                        None => 0,
                    });
                }
            }
            KeyCode::Enter => {
                if self.view_mode == ViewMode::Browser {
                    self.view_mode = ViewMode::Details;
                    self.tab_index = 1;
                }
            }
            KeyCode::Char('/') => {
                self.show_search_dialog = true;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.refresh().await?;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                // Clear namespace filter
                self.current_namespace = None;
                self.selected_entry = if self.entries.is_empty() { None } else { Some(0) };
            }
            _ => {}
        }
        Ok(())
    }

    fn perform_search(&mut self) {
        let query = self.search_query.to_lowercase();
        self.filtered_entries = self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                entry.key.to_lowercase().contains(&query)
                    || entry.value.to_lowercase().contains(&query)
                    || entry.namespace.to_lowercase().contains(&query)
                    || entry.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .map(|(idx, _)| idx)
            .collect();
        
        self.selected_entry = if self.filtered_entries.is_empty() { None } else { Some(0) };
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update memory entries in real implementation
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        info!("Refreshing memory browser");
        // In real implementation, reload from memory backend
        Ok(())
    }
}

fn format_time_ago(time: chrono::DateTime<chrono::Utc>) -> String {
    let duration = chrono::Utc::now().signed_duration_since(time);
    
    if duration.num_days() > 0 {
        format!("{}d ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{}m ago", duration.num_minutes())
    } else {
        format!("{}s ago", duration.num_seconds())
    }
}