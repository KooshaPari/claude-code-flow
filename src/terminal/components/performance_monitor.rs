//! Performance monitor component
//! Real-time system metrics and performance tracking

use anyhow::Result;
use tracing::info;

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
        Frame,
    },
};

pub struct PerformanceMonitor {
    cpu_history: Vec<u64>,
    memory_history: Vec<u64>,
    network_history: Vec<u64>,
    current_cpu: f64,
    current_memory: f64,
    current_network: u64,
}

impl PerformanceMonitor {
    pub async fn new() -> Result<Self> {
        info!("Initializing performance monitor");
        Ok(Self {
            cpu_history: vec![20, 25, 30, 35, 40, 45, 50, 55, 60, 65],
            memory_history: vec![30, 32, 34, 36, 38, 40, 42, 44, 46, 48],
            network_history: vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55],
            current_cpu: 0.65,
            current_memory: 0.48,
            current_network: 55,
        })
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),  // CPU
                Constraint::Length(5),  // Memory
                Constraint::Length(5),  // Network
                Constraint::Min(0),     // Charts
            ])
            .split(area);

        // CPU gauge
        let cpu_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("CPU Usage")
                    .title_style(Style::default().fg(Color::Green)),
            )
            .gauge_style(Style::default().fg(Color::Green))
            .percent((self.current_cpu * 100.0) as u16)
            .label(format!("{:.1}%", self.current_cpu * 100.0));
        f.render_widget(cpu_gauge, chunks[0]);

        // Memory gauge
        let memory_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory Usage")
                    .title_style(Style::default().fg(Color::Blue)),
            )
            .gauge_style(Style::default().fg(Color::Blue))
            .percent((self.current_memory * 100.0) as u16)
            .label(format!("{:.1}%", self.current_memory * 100.0));
        f.render_widget(memory_gauge, chunks[1]);

        // Network activity
        let network_paragraph = Paragraph::new(format!("Network: {} KB/s", self.current_network))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Network Activity")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(network_paragraph, chunks[2]);

        // Historical charts
        let chart_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(chunks[3]);

        // CPU history
        let cpu_sparkline = Sparkline::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("CPU History")
                    .title_style(Style::default().fg(Color::Green)),
            )
            .data(&self.cpu_history)
            .style(Style::default().fg(Color::Green));
        f.render_widget(cpu_sparkline, chart_chunks[0]);

        // Memory history
        let memory_sparkline = Sparkline::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory History")
                    .title_style(Style::default().fg(Color::Blue)),
            )
            .data(&self.memory_history)
            .style(Style::default().fg(Color::Blue));
        f.render_widget(memory_sparkline, chart_chunks[1]);

        // Network history
        let network_sparkline = Sparkline::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Network History")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .data(&self.network_history)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(network_sparkline, chart_chunks[2]);
    }

    #[cfg(feature = "terminal-ui")]
    pub async fn handle_key(&mut self, _key: KeyEvent) -> Result<()> {
        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        // Simulate performance data updates
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        self.current_cpu = rng.gen_range(0.0..1.0);
        self.current_memory = rng.gen_range(0.3..0.9);
        self.current_network = rng.gen_range(10..100);
        
        // Update history
        self.cpu_history.push((self.current_cpu * 100.0) as u64);
        self.memory_history.push((self.current_memory * 100.0) as u64);
        self.network_history.push(self.current_network);
        
        // Keep only last 20 entries
        if self.cpu_history.len() > 20 {
            self.cpu_history.remove(0);
        }
        if self.memory_history.len() > 20 {
            self.memory_history.remove(0);
        }
        if self.network_history.len() > 20 {
            self.network_history.remove(0);
        }
        
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        info!("Refreshing performance monitor");
        Ok(())
    }
}