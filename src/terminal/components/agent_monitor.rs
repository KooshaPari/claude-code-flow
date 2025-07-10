//! Agent monitoring component
//! Visual agent lifecycle and task assignment tracking

use anyhow::Result;
use std::time::Instant;
use tracing::info;

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect, Alignment},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{
            Block, Borders, List, ListItem, Table, Row, Cell, Gauge,
            Paragraph, Clear, Tabs,
        },
        Frame,
    },
};

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub status: AgentStatus,
    pub tasks: Vec<Task>,
    pub created_at: Instant,
    pub last_heartbeat: Instant,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentStatus {
    Starting,
    Idle,
    Working,
    Communicating,
    Error,
    Stopped,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentStatus::Starting => "Starting",
            AgentStatus::Idle => "Idle",
            AgentStatus::Working => "Working",
            AgentStatus::Communicating => "Communicating",
            AgentStatus::Error => "Error",
            AgentStatus::Stopped => "Stopped",
        }
    }

    #[cfg(feature = "terminal-ui")]
    pub fn color(&self) -> Color {
        match self {
            AgentStatus::Starting => Color::Yellow,
            AgentStatus::Idle => Color::Gray,
            AgentStatus::Working => Color::Green,
            AgentStatus::Communicating => Color::Cyan,
            AgentStatus::Error => Color::Red,
            AgentStatus::Stopped => Color::DarkGray,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            AgentStatus::Starting => "🔄",
            AgentStatus::Idle => "😴",
            AgentStatus::Working => "⚡",
            AgentStatus::Communicating => "💬",
            AgentStatus::Error => "❌",
            AgentStatus::Stopped => "⏹️",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub progress: f64,
    pub assigned_at: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
        }
    }

    #[cfg(feature = "terminal-ui")]
    pub fn color(&self) -> Color {
        match self {
            TaskStatus::Pending => Color::Yellow,
            TaskStatus::InProgress => Color::Blue,
            TaskStatus::Completed => Color::Green,
            TaskStatus::Failed => Color::Red,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ViewMode {
    List,
    Details,
    Tasks,
}

/// Agent monitoring component
pub struct AgentMonitor {
    agents: Vec<Agent>,
    selected_agent: Option<usize>,
    view_mode: ViewMode,
    tab_index: usize,
    show_spawn_dialog: bool,
    spawn_agent_type: String,
    spawn_agent_name: String,
}

impl AgentMonitor {
    pub async fn new() -> Result<Self> {
        info!("Initializing agent monitor");
        
        // Generate some mock agents for demonstration
        let mut agents = Vec::new();
        let agent_types = ["coordinator", "researcher", "implementer", "analyst", "tester"];
        
        for i in 0..3 {
            agents.push(Agent {
                id: format!("agent-{:03}", i + 1),
                name: format!("Agent-{}", i + 1),
                agent_type: agent_types[i % agent_types.len()].to_string(),
                status: if i == 0 { AgentStatus::Working } else { AgentStatus::Idle },
                tasks: vec![
                    Task {
                        id: format!("task-{}-001", i + 1),
                        description: "Process data analysis".to_string(),
                        status: TaskStatus::InProgress,
                        progress: 0.65,
                        assigned_at: Instant::now(),
                    }
                ],
                created_at: Instant::now(),
                last_heartbeat: Instant::now(),
                cpu_usage: 0.25,
                memory_usage: 128 * 1024 * 1024, // 128 MB
                capabilities: vec!["analysis".to_string(), "coordination".to_string()],
            });
        }
        
        Ok(Self {
            agents,
            selected_agent: Some(0),
            view_mode: ViewMode::List,
            tab_index: 0,
            show_spawn_dialog: false,
            spawn_agent_type: "coordinator".to_string(),
            spawn_agent_name: String::new(),
        })
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        if self.show_spawn_dialog {
            self.draw_spawn_dialog(f, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Status/Help
            ])
            .split(area);

        self.draw_tabs(f, chunks[0]);
        
        match self.view_mode {
            ViewMode::List => self.draw_agent_list(f, chunks[1]),
            ViewMode::Details => self.draw_agent_details(f, chunks[1]),
            ViewMode::Tasks => self.draw_task_view(f, chunks[1]),
        }
        
        self.draw_help_bar(f, chunks[2]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        let titles = vec!["List", "Details", "Tasks"];
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Agent Monitor")
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .select(self.tab_index);
        
        f.render_widget(tabs, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_agent_list(&self, f: &mut Frame, area: Rect) {
        let header = Row::new(vec![
            Cell::from("ID").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Name").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Type").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Tasks").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("CPU").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Memory").style(Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let rows: Vec<Row> = self.agents
            .iter()
            .enumerate()
            .map(|(i, agent)| {
                let style = if Some(i) == self.selected_agent {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(agent.id.clone()),
                    Cell::from(agent.name.clone()),
                    Cell::from(agent.agent_type.clone()),
                    Cell::from(Span::styled(
                        format!("{} {}", agent.status.icon(), agent.status.as_str()),
                        Style::default().fg(agent.status.color()),
                    )),
                    Cell::from(agent.tasks.len().to_string()),
                    Cell::from(format!("{:.1}%", agent.cpu_usage * 100.0)),
                    Cell::from(format!("{} MB", agent.memory_usage / 1024 / 1024)),
                ]).style(style)
            })
            .collect();

        let table = Table::new(rows, [
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(18),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(10),
        ])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Agents")
                .title_style(Style::default().fg(Color::Green)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_widget(table, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_agent_details(&self, f: &mut Frame, area: Rect) {
        if let Some(idx) = self.selected_agent {
            if let Some(agent) = self.agents.get(idx) {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);

                // Left side - Agent info
                let info_text = format!(
                    "Agent Information\\n\
                     ─────────────────\\n\
                     ID: {}\\n\
                     Name: {}\\n\
                     Type: {}\\n\
                     Status: {} {}\\n\
                     Created: {} ago\\n\
                     Last Heartbeat: {} ago\\n\
                     \\n\
                     Resource Usage\\n\
                     ──────────────\\n\
                     CPU Usage: {:.1}%\\n\
                     Memory: {} MB\\n\
                     \\n\
                     Capabilities\\n\
                     ────────────\\n\
                     {}",
                    agent.id,
                    agent.name,
                    agent.agent_type,
                    agent.status.icon(),
                    agent.status.as_str(),
                    format_duration(agent.created_at.elapsed()),
                    format_duration(agent.last_heartbeat.elapsed()),
                    agent.cpu_usage * 100.0,
                    agent.memory_usage / 1024 / 1024,
                    agent.capabilities.join(", ")
                );

                let info_paragraph = Paragraph::new(info_text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Details")
                            .title_style(Style::default().fg(Color::Yellow)),
                    )
                    .style(Style::default().fg(Color::White));

                f.render_widget(info_paragraph, chunks[0]);

                // Right side - Resource gauges
                let right_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(5),  // CPU gauge
                        Constraint::Length(5),  // Memory gauge
                        Constraint::Min(0),     // Task list
                    ])
                    .split(chunks[1]);

                // CPU gauge
                let cpu_gauge = Gauge::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("CPU Usage"),
                    )
                    .gauge_style(Style::default().fg(Color::Green))
                    .percent((agent.cpu_usage * 100.0) as u16)
                    .label(format!("{:.1}%", agent.cpu_usage * 100.0));
                f.render_widget(cpu_gauge, right_chunks[0]);

                // Memory gauge (assuming 1GB max for display)
                let memory_percent = ((agent.memory_usage as f64) / (1024.0 * 1024.0 * 1024.0) * 100.0).min(100.0);
                let memory_gauge = Gauge::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Memory Usage"),
                    )
                    .gauge_style(Style::default().fg(Color::Blue))
                    .percent(memory_percent as u16)
                    .label(format!("{} MB", agent.memory_usage / 1024 / 1024));
                f.render_widget(memory_gauge, right_chunks[1]);

                // Tasks
                let task_items: Vec<ListItem> = agent.tasks
                    .iter()
                    .map(|task| {
                        ListItem::new(Line::from(vec![
                            Span::styled(
                                format!("{:<20}", task.description),
                                Style::default().fg(Color::White),
                            ),
                            Span::styled(
                                format!("{:<12}", task.status.as_str()),
                                Style::default().fg(task.status.color()),
                            ),
                            Span::styled(
                                format!("{:.0}%", task.progress * 100.0),
                                Style::default().fg(Color::Cyan),
                            ),
                        ]))
                    })
                    .collect();

                let task_list = List::new(task_items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Active Tasks")
                            .title_style(Style::default().fg(Color::Magenta)),
                    );

                f.render_widget(task_list, right_chunks[2]);
            }
        } else {
            let no_selection = Paragraph::new("No agent selected")
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
    fn draw_task_view(&self, f: &mut Frame, area: Rect) {
        let all_tasks: Vec<(String, Task)> = self.agents
            .iter()
            .flat_map(|agent| {
                agent.tasks.iter().map(|task| (agent.name.clone(), task.clone()))
            })
            .collect();

        let header = Row::new(vec![
            Cell::from("Agent").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Task ID").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Description").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Progress").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Duration").style(Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let rows: Vec<Row> = all_tasks
            .iter()
            .map(|(agent_name, task)| {
                Row::new(vec![
                    Cell::from(agent_name.clone()),
                    Cell::from(task.id.clone()),
                    Cell::from(task.description.clone()),
                    Cell::from(Span::styled(
                        task.status.as_str(),
                        Style::default().fg(task.status.color()),
                    )),
                    Cell::from(format!("{:.0}%", task.progress * 100.0)),
                    Cell::from(format_duration(task.assigned_at.elapsed())),
                ])
            })
            .collect();

        let table = Table::new(rows, [
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("All Tasks ({})", all_tasks.len()))
                .title_style(Style::default().fg(Color::Magenta)),
        );

        f.render_widget(table, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_spawn_dialog(&self, f: &mut Frame, area: Rect) {
        let popup_area = self.centered_rect(60, 40, area);
        
        let dialog_text = format!(
            "Spawn New Agent\\n\
             ──────────────\\n\
             \\n\
             Agent Type: {}\\n\
             Agent Name: {}\\n\
             \\n\
             Available Types:\\n\
             - coordinator\\n\
             - researcher\\n\
             - implementer\\n\
             - analyst\\n\
             - tester\\n\
             \\n\
             Press Enter to spawn, Esc to cancel",
            self.spawn_agent_type,
            if self.spawn_agent_name.is_empty() { "<auto-generated>" } else { &self.spawn_agent_name }
        );

        let dialog = Paragraph::new(dialog_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Spawn Agent")
                    .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);

        f.render_widget(Clear, popup_area);
        f.render_widget(dialog, popup_area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_help_bar(&self, f: &mut Frame, area: Rect) {
        let help_text = match self.view_mode {
            ViewMode::List => "↑↓: Select | Tab: Switch View | S: Spawn | K: Kill | Enter: Details | Esc: Back",
            ViewMode::Details => "Tab: Switch View | R: Refresh | Esc: Back",
            ViewMode::Tasks => "Tab: Switch View | R: Refresh | Esc: Back",
        };

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(help, area);
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
        if self.show_spawn_dialog {
            match key.code {
                KeyCode::Esc => {
                    self.show_spawn_dialog = false;
                    self.spawn_agent_name.clear();
                }
                KeyCode::Enter => {
                    self.spawn_agent().await?;
                    self.show_spawn_dialog = false;
                    self.spawn_agent_name.clear();
                }
                _ => {}
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Tab => {
                self.tab_index = (self.tab_index + 1) % 3;
                self.view_mode = match self.tab_index {
                    0 => ViewMode::List,
                    1 => ViewMode::Details,
                    2 => ViewMode::Tasks,
                    _ => ViewMode::List,
                };
            }
            KeyCode::Up => {
                if !self.agents.is_empty() {
                    self.selected_agent = Some(match self.selected_agent {
                        Some(idx) if idx > 0 => idx - 1,
                        Some(_) => self.agents.len() - 1,
                        None => 0,
                    });
                }
            }
            KeyCode::Down => {
                if !self.agents.is_empty() {
                    self.selected_agent = Some(match self.selected_agent {
                        Some(idx) if idx < self.agents.len() - 1 => idx + 1,
                        Some(_) => 0,
                        None => 0,
                    });
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.show_spawn_dialog = true;
            }
            KeyCode::Char('k') | KeyCode::Char('K') => {
                if let Some(idx) = self.selected_agent {
                    self.kill_agent(idx).await?;
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.refresh().await?;
            }
            KeyCode::Enter => {
                if self.view_mode == ViewMode::List {
                    self.view_mode = ViewMode::Details;
                    self.tab_index = 1;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn spawn_agent(&mut self) -> Result<()> {
        info!("Spawning new agent of type: {}", self.spawn_agent_type);
        
        let new_id = format!("agent-{:03}", self.agents.len() + 1);
        let new_name = if self.spawn_agent_name.is_empty() {
            format!("{}-{}", self.spawn_agent_type.to_uppercase(), self.agents.len() + 1)
        } else {
            self.spawn_agent_name.clone()
        };

        let new_agent = Agent {
            id: new_id,
            name: new_name,
            agent_type: self.spawn_agent_type.clone(),
            status: AgentStatus::Starting,
            tasks: Vec::new(),
            created_at: Instant::now(),
            last_heartbeat: Instant::now(),
            cpu_usage: 0.0,
            memory_usage: 64 * 1024 * 1024, // 64 MB
            capabilities: vec!["basic".to_string()],
        };

        self.agents.push(new_agent);
        self.selected_agent = Some(self.agents.len() - 1);

        Ok(())
    }

    async fn kill_agent(&mut self, index: usize) -> Result<()> {
        if index < self.agents.len() {
            let agent = &self.agents[index];
            info!("Killing agent: {}", agent.id);
            
            self.agents.remove(index);
            
            // Adjust selected index
            if self.agents.is_empty() {
                self.selected_agent = None;
            } else if index >= self.agents.len() {
                self.selected_agent = Some(self.agents.len() - 1);
            }
        }
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update agent statuses and simulate activity
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for agent in &mut self.agents {
            // Update heartbeat
            agent.last_heartbeat = Instant::now();
            
            // Simulate status changes
            if rng.gen_bool(0.1) {
                agent.status = match agent.status {
                    AgentStatus::Starting => AgentStatus::Idle,
                    AgentStatus::Idle => if rng.gen_bool(0.3) { AgentStatus::Working } else { AgentStatus::Idle },
                    AgentStatus::Working => if rng.gen_bool(0.2) { AgentStatus::Idle } else { AgentStatus::Working },
                    AgentStatus::Communicating => AgentStatus::Idle,
                    AgentStatus::Error => AgentStatus::Idle,
                    AgentStatus::Stopped => AgentStatus::Stopped,
                };
            }
            
            // Update resource usage
            agent.cpu_usage = rng.gen_range(0.0..1.0);
            
            // Update task progress
            for task in &mut agent.tasks {
                if task.status == TaskStatus::InProgress {
                    task.progress = (task.progress + rng.gen_range(0.01..0.05)).min(1.0);
                    if task.progress >= 1.0 {
                        task.status = TaskStatus::Completed;
                    }
                }
            }
        }
        
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        info!("Refreshing agent monitor");
        // In a real implementation, this would fetch fresh data from the agent manager
        self.update().await
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else {
        format!("{}h", secs / 3600)
    }
}