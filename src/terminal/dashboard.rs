//! Real-time dashboard component
//! Displays live metrics, agents, and system status

use anyhow::Result;
use std::time::{Duration, Instant};
use tracing::{info, warn};

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect, Alignment},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{
            Block, Borders, Gauge, List, ListItem, Paragraph, Sparkline, Table, Row, Cell,
            BarChart, Clear,
        },
        symbols,
        Frame,
    },
};

use crate::config;

/// Dashboard data structures
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub uptime: Duration,
    pub active_connections: u32,
}

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub status: AgentStatus,
    pub tasks_completed: u32,
    pub tasks_pending: u32,
    pub last_activity: Instant,
    pub cpu_usage: f64,
    pub memory_usage: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentStatus {
    Idle,
    Active,
    Busy,
    Error,
    Disconnected,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentStatus::Idle => "Idle",
            AgentStatus::Active => "Active",
            AgentStatus::Busy => "Busy",
            AgentStatus::Error => "Error",
            AgentStatus::Disconnected => "Disconnected",
        }
    }

    #[cfg(feature = "terminal-ui")]
    pub fn color(&self) -> Color {
        match self {
            AgentStatus::Idle => Color::Gray,
            AgentStatus::Active => Color::Green,
            AgentStatus::Busy => Color::Yellow,
            AgentStatus::Error => Color::Red,
            AgentStatus::Disconnected => Color::DarkGray,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwarmMetrics {
    pub total_agents: u32,
    pub active_agents: u32,
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub failed_tasks: u32,
    pub avg_task_time: Duration,
    pub consensus_rate: f64,
    pub coordination_latency: Duration,
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_entries: u64,
    pub cache_size: u64,
    pub hit_rate: f64,
    pub storage_backend: String,
}

/// Real-time dashboard component
pub struct RealTimeDashboard {
    config: config::UiConfig,
    
    // Data
    system_metrics: SystemMetrics,
    agents: Vec<AgentInfo>,
    swarm_metrics: SwarmMetrics,
    memory_stats: MemoryStats,
    
    // UI state
    paused: bool,
    refresh_rate: Duration,
    last_update: Instant,
    selected_agent: Option<usize>,
    show_details: bool,
    
    // Historical data for charts
    cpu_history: Vec<f64>,
    memory_history: Vec<f64>,
    task_completion_history: Vec<u64>,
    max_history_points: usize,
}

impl RealTimeDashboard {
    pub async fn new(config: &config::UiConfig) -> Result<Self> {
        info!("Initializing real-time dashboard");
        
        let system_metrics = SystemMetrics {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_rx: 0,
            network_tx: 0,
            uptime: Duration::from_secs(0),
            active_connections: 0,
        };
        
        let swarm_metrics = SwarmMetrics {
            total_agents: 0,
            active_agents: 0,
            total_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            avg_task_time: Duration::from_secs(0),
            consensus_rate: 0.0,
            coordination_latency: Duration::from_millis(0),
        };
        
        let memory_stats = MemoryStats {
            total_entries: 0,
            cache_size: 0,
            hit_rate: 0.0,
            storage_backend: "SQLite + Memory".to_string(),
        };
        
        Ok(Self {
            config: config.clone(),
            system_metrics,
            agents: Vec::new(),
            swarm_metrics,
            memory_stats,
            paused: false,
            refresh_rate: Duration::from_millis(1000),
            last_update: Instant::now(),
            selected_agent: None,
            show_details: false,
            cpu_history: Vec::new(),
            memory_history: Vec::new(),
            task_completion_history: Vec::new(),
            max_history_points: 60, // 1 minute of data at 1 second intervals
        })
    }

    /// Run dashboard in standalone mode
    pub async fn run(&self) -> Result<()> {
        info!("Starting dashboard in standalone mode");
        Ok(())
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        if self.show_details && self.selected_agent.is_some() {
            self.draw_agent_details(f, area);
            return;
        }

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(9),  // System overview
                Constraint::Min(0),     // Content area
                Constraint::Length(3),  // Status bar
            ])
            .split(area);

        self.draw_system_overview(f, main_chunks[0]);
        self.draw_content_area(f, main_chunks[1]);
        self.draw_status_bar(f, main_chunks[2]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_system_overview(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // CPU Usage
        let cpu_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("CPU Usage")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .gauge_style(Style::default().fg(Color::Green))
            .percent((self.system_metrics.cpu_usage * 100.0) as u16)
            .label(format!("{:.1}%", self.system_metrics.cpu_usage * 100.0));
        f.render_widget(cpu_gauge, chunks[0]);

        // Memory Usage
        let memory_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory Usage")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .gauge_style(Style::default().fg(Color::Blue))
            .percent((self.system_metrics.memory_usage * 100.0) as u16)
            .label(format!("{:.1}%", self.system_metrics.memory_usage * 100.0));
        f.render_widget(memory_gauge, chunks[1]);

        // Active Agents
        let agents_block = Block::default()
            .borders(Borders::ALL)
            .title("Agents")
            .title_style(Style::default().fg(Color::Cyan));
        let agents_text = Paragraph::new(format!(
            "Total: {}\\nActive: {}\\nIdle: {}",
            self.swarm_metrics.total_agents,
            self.swarm_metrics.active_agents,
            self.swarm_metrics.total_agents - self.swarm_metrics.active_agents
        ))
        .block(agents_block)
        .alignment(Alignment::Center);
        f.render_widget(agents_text, chunks[2]);

        // Tasks
        let tasks_block = Block::default()
            .borders(Borders::ALL)
            .title("Tasks")
            .title_style(Style::default().fg(Color::Cyan));
        let tasks_text = Paragraph::new(format!(
            "Total: {}\\nCompleted: {}\\nFailed: {}",
            self.swarm_metrics.total_tasks,
            self.swarm_metrics.completed_tasks,
            self.swarm_metrics.failed_tasks
        ))
        .block(tasks_block)
        .alignment(Alignment::Center);
        f.render_widget(tasks_text, chunks[3]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_content_area(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),  // Agent list
                Constraint::Percentage(60),  // Charts and metrics
            ])
            .split(area);

        self.draw_agent_list(f, chunks[0]);
        self.draw_charts_area(f, chunks[1]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_agent_list(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.agents
            .iter()
            .enumerate()
            .map(|(i, agent)| {
                let status_color = agent.status.color();
                let indicator = if Some(i) == self.selected_agent { "► " } else { "  " };
                
                ListItem::new(Line::from(vec![
                    Span::styled(indicator, Style::default().fg(Color::Yellow)),
                    Span::styled(format!("{:<12}", agent.name), Style::default().fg(Color::White)),
                    Span::styled(format!("{:<10}", agent.agent_type), Style::default().fg(Color::Gray)),
                    Span::styled(format!("{:<12}", agent.status.as_str()), Style::default().fg(status_color)),
                    Span::styled(format!("{}/{}", agent.tasks_completed, agent.tasks_pending), Style::default().fg(Color::Cyan)),
                ]))
            })
            .collect();

        let agent_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Agents ({}) - Use ↑↓ to select, Enter for details", self.agents.len()))
                    .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("> ");

        f.render_widget(agent_list, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_charts_area(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),  // Performance charts
                Constraint::Percentage(50),  // Metrics and stats
            ])
            .split(area);

        self.draw_performance_charts(f, chunks[0]);
        self.draw_metrics_panel(f, chunks[1]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_performance_charts(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // CPU history sparkline
        if !self.cpu_history.is_empty() {
            let cpu_data: Vec<u64> = self.cpu_history
                .iter()
                .map(|&x| (x * 100.0) as u64)
                .collect();
            
            let cpu_sparkline = Sparkline::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("CPU History")
                        .title_style(Style::default().fg(Color::Green)),
                )
                .data(&cpu_data)
                .style(Style::default().fg(Color::Green));
            f.render_widget(cpu_sparkline, chunks[0]);
        }

        // Memory history sparkline
        if !self.memory_history.is_empty() {
            let memory_data: Vec<u64> = self.memory_history
                .iter()
                .map(|&x| (x * 100.0) as u64)
                .collect();
            
            let memory_sparkline = Sparkline::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Memory History")
                        .title_style(Style::default().fg(Color::Blue)),
                )
                .data(&memory_data)
                .style(Style::default().fg(Color::Blue));
            f.render_widget(memory_sparkline, chunks[1]);
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_metrics_panel(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Swarm metrics
        let swarm_metrics_text = format!(
            "Swarm Metrics\\n\
             ─────────────\\n\
             Consensus Rate: {:.1}%\\n\
             Avg Task Time: {:.1}s\\n\
             Coordination Latency: {}ms\\n\
             Success Rate: {:.1}%",
            self.swarm_metrics.consensus_rate * 100.0,
            self.swarm_metrics.avg_task_time.as_secs_f64(),
            self.swarm_metrics.coordination_latency.as_millis(),
            if self.swarm_metrics.total_tasks > 0 {
                (self.swarm_metrics.completed_tasks as f64 / self.swarm_metrics.total_tasks as f64) * 100.0
            } else { 0.0 }
        );

        let swarm_panel = Paragraph::new(swarm_metrics_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Swarm")
                    .title_style(Style::default().fg(Color::Magenta)),
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(swarm_panel, chunks[0]);

        // Memory metrics
        let memory_metrics_text = format!(
            "Memory System\\n\
             ─────────────\\n\
             Total Entries: {}\\n\
             Cache Size: {} MB\\n\
             Hit Rate: {:.1}%\\n\
             Backend: {}",
            self.memory_stats.total_entries,
            self.memory_stats.cache_size / 1024 / 1024,
            self.memory_stats.hit_rate * 100.0,
            self.memory_stats.storage_backend
        );

        let memory_panel = Paragraph::new(memory_metrics_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory")
                    .title_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(memory_panel, chunks[1]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_agent_details(&self, f: &mut Frame, area: Rect) {
        if let Some(idx) = self.selected_agent {
            if let Some(agent) = self.agents.get(idx) {
                let popup_area = self.centered_rect(80, 80, area);
                
                let details_text = format!(
                    "Agent Details\\n\
                     ─────────────\\n\
                     ID: {}\\n\
                     Name: {}\\n\
                     Type: {}\\n\
                     Status: {}\\n\
                     Tasks Completed: {}\\n\
                     Tasks Pending: {}\\n\
                     CPU Usage: {:.1}%\\n\
                     Memory Usage: {} MB\\n\
                     Last Activity: {} ago\\n\
                     \\n\
                     Press Esc to close",
                    agent.id,
                    agent.name,
                    agent.agent_type,
                    agent.status.as_str(),
                    agent.tasks_completed,
                    agent.tasks_pending,
                    agent.cpu_usage * 100.0,
                    agent.memory_usage / 1024 / 1024,
                    format_duration(agent.last_activity.elapsed())
                );

                let details_popup = Paragraph::new(details_text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Agent Details")
                            .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    )
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Left);

                f.render_widget(Clear, popup_area);
                f.render_widget(details_popup, popup_area);
            }
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        let status_text = if self.paused {
            "PAUSED - Press Space to resume | R: Refresh | Q: Quit | ↑↓: Select Agent | Enter: Details"
        } else {
            "RUNNING - Space: Pause | R: Refresh | Q: Quit | ↑↓: Select Agent | Enter: Details"
        };

        let status_color = if self.paused { Color::Red } else { Color::Green };
        
        let status_paragraph = Paragraph::new(status_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Status")
                    .title_style(Style::default().fg(status_color)),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        f.render_widget(status_paragraph, area);
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
        if self.show_details {
            match key.code {
                KeyCode::Esc => {
                    self.show_details = false;
                }
                _ => {}
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Char(' ') => {
                self.paused = !self.paused;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.refresh_data().await?;
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
            KeyCode::Enter => {
                if self.selected_agent.is_some() {
                    self.show_details = true;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        if !self.paused && self.last_update.elapsed() >= self.refresh_rate {
            self.refresh_data().await?;
            self.last_update = Instant::now();
        }
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        self.refresh_data().await
    }

    async fn refresh_data(&mut self) -> Result<()> {
        // Update system metrics
        self.update_system_metrics().await?;
        
        // Update agent information
        self.update_agent_data().await?;
        
        // Update swarm metrics
        self.update_swarm_metrics().await?;
        
        // Update memory stats
        self.update_memory_stats().await?;
        
        // Update historical data
        self.update_history();
        
        Ok(())
    }

    async fn update_system_metrics(&mut self) -> Result<()> {
        // Simulate system metrics - in real implementation, get from system
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        self.system_metrics.cpu_usage = rng.gen_range(0.0..1.0);
        self.system_metrics.memory_usage = rng.gen_range(0.3..0.9);
        self.system_metrics.disk_usage = rng.gen_range(0.1..0.8);
        self.system_metrics.network_rx = rng.gen_range(1000..100000);
        self.system_metrics.network_tx = rng.gen_range(1000..100000);
        self.system_metrics.uptime = Duration::from_secs(rng.gen_range(3600..86400));
        self.system_metrics.active_connections = rng.gen_range(1..20);
        
        Ok(())
    }

    async fn update_agent_data(&mut self) -> Result<()> {
        // Simulate agent data - in real implementation, get from agent manager
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Generate some mock agents if empty
        if self.agents.is_empty() {
            for i in 0..5 {
                let agent_types = ["coordinator", "researcher", "implementer", "analyst", "monitor"];
                let statuses = [AgentStatus::Idle, AgentStatus::Active, AgentStatus::Busy];
                
                self.agents.push(AgentInfo {
                    id: format!("agent-{:03}", i + 1),
                    name: format!("Agent-{}", i + 1),
                    agent_type: agent_types[i % agent_types.len()].to_string(),
                    status: statuses[rng.gen_range(0..statuses.len())].clone(),
                    tasks_completed: rng.gen_range(0..50),
                    tasks_pending: rng.gen_range(0..10),
                    last_activity: Instant::now() - Duration::from_secs(rng.gen_range(0..3600)),
                    cpu_usage: rng.gen_range(0.0..1.0),
                    memory_usage: rng.gen_range(10..500) * 1024 * 1024, // MB in bytes
                });
            }
        } else {
            // Update existing agents
            for agent in &mut self.agents {
                // Occasionally change status
                if rng.gen_bool(0.1) {
                    let statuses = [AgentStatus::Idle, AgentStatus::Active, AgentStatus::Busy];
                    agent.status = statuses[rng.gen_range(0..statuses.len())].clone();
                }
                
                // Update metrics
                agent.cpu_usage = rng.gen_range(0.0..1.0);
                if rng.gen_bool(0.3) {
                    agent.tasks_completed += 1;
                }
                if rng.gen_bool(0.2) {
                    agent.tasks_pending = rng.gen_range(0..5);
                }
            }
        }
        
        Ok(())
    }

    async fn update_swarm_metrics(&mut self) -> Result<()> {
        // Calculate swarm metrics from agent data
        self.swarm_metrics.total_agents = self.agents.len() as u32;
        self.swarm_metrics.active_agents = self.agents
            .iter()
            .filter(|a| matches!(a.status, AgentStatus::Active | AgentStatus::Busy))
            .count() as u32;
        
        self.swarm_metrics.completed_tasks = self.agents
            .iter()
            .map(|a| a.tasks_completed)
            .sum();
        
        // Simulate other metrics
        use rand::Rng;
        let mut rng = rand::thread_rng();
        self.swarm_metrics.consensus_rate = rng.gen_range(0.8..1.0);
        self.swarm_metrics.avg_task_time = Duration::from_secs(rng.gen_range(30..300));
        self.swarm_metrics.coordination_latency = Duration::from_millis(rng.gen_range(50..500));
        
        Ok(())
    }

    async fn update_memory_stats(&mut self) -> Result<()> {
        // Simulate memory stats
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        self.memory_stats.total_entries = rng.gen_range(1000..10000);
        self.memory_stats.cache_size = rng.gen_range(50..200) * 1024 * 1024; // MB in bytes
        self.memory_stats.hit_rate = rng.gen_range(0.85..0.99);
        
        Ok(())
    }

    fn update_history(&mut self) {
        // Add current values to history
        self.cpu_history.push(self.system_metrics.cpu_usage);
        self.memory_history.push(self.system_metrics.memory_usage);
        self.task_completion_history.push(self.swarm_metrics.completed_tasks as u64);
        
        // Trim history to max size
        if self.cpu_history.len() > self.max_history_points {
            self.cpu_history.remove(0);
        }
        if self.memory_history.len() > self.max_history_points {
            self.memory_history.remove(0);
        }
        if self.task_completion_history.len() > self.max_history_points {
            self.task_completion_history.remove(0);
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}