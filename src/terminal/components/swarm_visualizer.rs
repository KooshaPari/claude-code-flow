//! Swarm visualization component
//! ASCII art swarm topology display and real-time coordination

use anyhow::Result;
use tracing::info;

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect, Alignment},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph, List, ListItem, canvas::Canvas},
        symbols,
        Frame,
    },
};

pub struct SwarmVisualizer {
    topology: String,
    agents: Vec<(f64, f64, String)>, // x, y, name
    connections: Vec<((f64, f64), (f64, f64))>,
    zoom_level: f64,
    center_x: f64,
    center_y: f64,
}

impl SwarmVisualizer {
    pub async fn new() -> Result<Self> {
        info!("Initializing swarm visualizer");
        
        Ok(Self {
            topology: "hierarchical".to_string(),
            agents: vec![
                (50.0, 30.0, "Queen".to_string()),
                (30.0, 60.0, "Worker-1".to_string()),
                (70.0, 60.0, "Worker-2".to_string()),
                (50.0, 80.0, "Coordinator".to_string()),
            ],
            connections: vec![
                ((50.0, 30.0), (30.0, 60.0)),
                ((50.0, 30.0), (70.0, 60.0)),
                ((30.0, 60.0), (50.0, 80.0)),
                ((70.0, 60.0), (50.0, 80.0)),
            ],
            zoom_level: 1.0,
            center_x: 50.0,
            center_y: 50.0,
        })
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        self.draw_swarm_canvas(f, chunks[0]);
        self.draw_info_panel(f, chunks[1]);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_swarm_canvas(&self, f: &mut Frame, area: Rect) {
        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Swarm Topology")
                    .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            )
            .paint(|ctx| {
                // Draw connections
                for ((x1, y1), (x2, y2)) in &self.connections {
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: *x1,
                        y1: *y1,
                        x2: *x2,
                        y2: *y2,
                        color: Color::Gray,
                    });
                }
                
                // Draw agents
                for (x, y, name) in &self.agents {
                    ctx.draw(&ratatui::widgets::canvas::Points {
                        coords: &[(*x, *y)],
                        color: if name == "Queen" { Color::Yellow } else { Color::Green },
                    });
                }
            })
            .marker(symbols::Marker::Braille)
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0]);

        f.render_widget(canvas, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_info_panel(&self, f: &mut Frame, area: Rect) {
        let info_text = format!(
            "Swarm Information\\n\
             ─────────────────\\n\
             Topology: {}\\n\
             Agents: {}\\n\
             Connections: {}\\n\
             Zoom: {:.1}x\\n\
             \\n\
             Controls:\\n\
             +/- : Zoom\\n\
             Arrows: Pan\\n\
             R: Reset view\\n\
             T: Change topology",
            self.topology,
            self.agents.len(),
            self.connections.len(),
            self.zoom_level
        );

        let info_panel = Paragraph::new(info_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Info")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(info_panel, area);
    }

    #[cfg(feature = "terminal-ui")]
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.zoom_level = (self.zoom_level * 1.2).min(5.0);
            }
            KeyCode::Char('-') => {
                self.zoom_level = (self.zoom_level / 1.2).max(0.2);
            }
            KeyCode::Left => {
                self.center_x -= 5.0;
            }
            KeyCode::Right => {
                self.center_x += 5.0;
            }
            KeyCode::Up => {
                self.center_y -= 5.0;
            }
            KeyCode::Down => {
                self.center_y += 5.0;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.center_x = 50.0;
                self.center_y = 50.0;
                self.zoom_level = 1.0;
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.topology = match self.topology.as_str() {
                    "hierarchical" => "mesh".to_string(),
                    "mesh" => "ring".to_string(),
                    "ring" => "star".to_string(),
                    _ => "hierarchical".to_string(),
                };
                self.update_topology();
            }
            _ => {}
        }
        Ok(())
    }

    fn update_topology(&mut self) {
        // Update agent positions based on topology
        match self.topology.as_str() {
            "mesh" => {
                // Arrange in a grid
                self.agents = vec![
                    (25.0, 25.0, "Agent-1".to_string()),
                    (75.0, 25.0, "Agent-2".to_string()),
                    (25.0, 75.0, "Agent-3".to_string()),
                    (75.0, 75.0, "Agent-4".to_string()),
                ];
                // Full mesh connections
                self.connections = vec![
                    ((25.0, 25.0), (75.0, 25.0)),
                    ((25.0, 25.0), (25.0, 75.0)),
                    ((25.0, 25.0), (75.0, 75.0)),
                    ((75.0, 25.0), (25.0, 75.0)),
                    ((75.0, 25.0), (75.0, 75.0)),
                    ((25.0, 75.0), (75.0, 75.0)),
                ];
            }
            "ring" => {
                // Arrange in a circle
                self.agents = vec![
                    (50.0, 20.0, "Agent-1".to_string()),
                    (80.0, 50.0, "Agent-2".to_string()),
                    (50.0, 80.0, "Agent-3".to_string()),
                    (20.0, 50.0, "Agent-4".to_string()),
                ];
                self.connections = vec![
                    ((50.0, 20.0), (80.0, 50.0)),
                    ((80.0, 50.0), (50.0, 80.0)),
                    ((50.0, 80.0), (20.0, 50.0)),
                    ((20.0, 50.0), (50.0, 20.0)),
                ];
            }
            "star" => {
                // Central hub with spokes
                self.agents = vec![
                    (50.0, 50.0, "Hub".to_string()),
                    (50.0, 20.0, "Agent-1".to_string()),
                    (80.0, 50.0, "Agent-2".to_string()),
                    (50.0, 80.0, "Agent-3".to_string()),
                    (20.0, 50.0, "Agent-4".to_string()),
                ];
                self.connections = vec![
                    ((50.0, 50.0), (50.0, 20.0)),
                    ((50.0, 50.0), (80.0, 50.0)),
                    ((50.0, 50.0), (50.0, 80.0)),
                    ((50.0, 50.0), (20.0, 50.0)),
                ];
            }
            _ => {
                // Default hierarchical
                self.agents = vec![
                    (50.0, 30.0, "Queen".to_string()),
                    (30.0, 60.0, "Worker-1".to_string()),
                    (70.0, 60.0, "Worker-2".to_string()),
                    (50.0, 80.0, "Coordinator".to_string()),
                ];
                self.connections = vec![
                    ((50.0, 30.0), (30.0, 60.0)),
                    ((50.0, 30.0), (70.0, 60.0)),
                    ((30.0, 60.0), (50.0, 80.0)),
                    ((70.0, 60.0), (50.0, 80.0)),
                ];
            }
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        // Animate agent positions or update connections
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        info!("Refreshing swarm visualizer");
        Ok(())
    }
}