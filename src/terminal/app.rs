//! Main terminal application coordinator
//! Manages different UI modes and routing between components

use anyhow::Result;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, error};

#[cfg(feature = "terminal-ui")]
use {
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    ratatui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs},
        Frame, Terminal,
    },
    std::io,
};

use crate::config;
use super::{
    dashboard::RealTimeDashboard,
    repl::InteractiveRepl,
    components::{
        agent_monitor::AgentMonitor,
        memory_browser::MemoryBrowser,
        swarm_visualizer::SwarmVisualizer,
        progress_indicators::ProgressIndicators,
        log_viewer::LogViewer,
        config_editor::ConfigEditor,
        help_system::HelpSystem,
        performance_monitor::PerformanceMonitor,
    },
    events::EventHandler,
    themes::Theme,
};

/// Application modes
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Dashboard,
    Repl,
    AgentMonitor,
    MemoryBrowser,
    SwarmVisualizer,
    Progress,
    Logs,
    Config,
    Help,
    Performance,
}

impl AppMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppMode::Dashboard => "Dashboard",
            AppMode::Repl => "REPL",
            AppMode::AgentMonitor => "Agents",
            AppMode::MemoryBrowser => "Memory",
            AppMode::SwarmVisualizer => "Swarm",
            AppMode::Progress => "Progress",
            AppMode::Logs => "Logs",
            AppMode::Config => "Config",
            AppMode::Help => "Help",
            AppMode::Performance => "Performance",
        }
    }

    pub fn all_modes() -> Vec<AppMode> {
        vec![
            AppMode::Dashboard,
            AppMode::Repl,
            AppMode::AgentMonitor,
            AppMode::MemoryBrowser,
            AppMode::SwarmVisualizer,
            AppMode::Progress,
            AppMode::Logs,
            AppMode::Config,
            AppMode::Help,
            AppMode::Performance,
        ]
    }
}

/// Main terminal application
pub struct TerminalApp {
    config: config::UiConfig,
    current_mode: AppMode,
    mode_index: usize,
    should_quit: bool,
    last_tick: Instant,
    tick_rate: Duration,
    
    // Components
    dashboard: RealTimeDashboard,
    repl: InteractiveRepl,
    agent_monitor: AgentMonitor,
    memory_browser: MemoryBrowser,
    swarm_visualizer: SwarmVisualizer,
    progress_indicators: ProgressIndicators,
    log_viewer: LogViewer,
    config_editor: ConfigEditor,
    help_system: HelpSystem,
    performance_monitor: PerformanceMonitor,
    
    // UI state
    theme: Theme,
    event_handler: EventHandler,
    show_help_popup: bool,
}

impl TerminalApp {
    pub async fn new(config: &config::UiConfig) -> Result<Self> {
        info!("Initializing terminal application");
        
        let theme = Theme::default();
        let event_handler = EventHandler::new();
        
        Ok(Self {
            config: config.clone(),
            current_mode: AppMode::Dashboard,
            mode_index: 0,
            should_quit: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(250),
            
            // Initialize components
            dashboard: RealTimeDashboard::new(config).await?,
            repl: InteractiveRepl::new(config).await?,
            agent_monitor: AgentMonitor::new().await?,
            memory_browser: MemoryBrowser::new().await?,
            swarm_visualizer: SwarmVisualizer::new().await?,
            progress_indicators: ProgressIndicators::new().await?,
            log_viewer: LogViewer::new().await?,
            config_editor: ConfigEditor::new(config).await?,
            help_system: HelpSystem::new().await?,
            performance_monitor: PerformanceMonitor::new().await?,
            
            theme,
            event_handler,
            show_help_popup: false,
        })
    }

    /// Run the terminal application
    #[cfg(feature = "terminal-ui")]
    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main application loop
        let result = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        match result {
            Ok(_) => info!("Terminal application closed gracefully"),
            Err(e) => error!("Terminal application error: {}", e),
        }

        result
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub async fn run(&mut self) -> Result<()> {
        info!("Terminal UI not available, falling back to basic mode");
        loop {
            println!("Claude Flow Terminal (Basic Mode)");
            println!("Terminal UI features require 'terminal-ui' feature to be enabled");
            sleep(Duration::from_secs(5)).await;
            break;
        }
        Ok(())
    }

    #[cfg(feature = "terminal-ui")]
    async fn run_app<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            // Draw UI
            terminal.draw(|f| self.draw(f))?;

            // Handle events
            let timeout = self.tick_rate
                .checked_sub(self.last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(key).await?;
                }
            }

            // Update components on tick
            if self.last_tick.elapsed() >= self.tick_rate {
                self.on_tick().await?;
                self.last_tick = Instant::now();
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    #[cfg(feature = "terminal-ui")]
    fn draw(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.area());

        self.draw_tabs(f, chunks[0]);
        self.draw_content(f, chunks[1]);

        if self.show_help_popup {
            self.draw_help_popup(f, f.area());
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        let titles: Vec<Line> = AppMode::all_modes()
            .iter()
            .map(|mode| Line::from(Span::styled(mode.as_str(), Style::default())))
            .collect();

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Claude Flow Terminal UI")
                    .title_style(self.theme.title_style()),
            )
            .style(self.theme.tab_style())
            .highlight_style(self.theme.tab_selected_style())
            .select(self.mode_index);

        f.render_widget(tabs, area);
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_content(&mut self, f: &mut Frame, area: Rect) {
        match self.current_mode {
            AppMode::Dashboard => self.dashboard.draw(f, area),
            AppMode::Repl => self.repl.draw(f, area),
            AppMode::AgentMonitor => self.agent_monitor.draw(f, area),
            AppMode::MemoryBrowser => self.memory_browser.draw(f, area),
            AppMode::SwarmVisualizer => self.swarm_visualizer.draw(f, area),
            AppMode::Progress => self.progress_indicators.draw(f, area),
            AppMode::Logs => self.log_viewer.draw(f, area),
            AppMode::Config => self.config_editor.draw(f, area),
            AppMode::Help => self.help_system.draw(f, area),
            AppMode::Performance => self.performance_monitor.draw(f, area),
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_help_popup(&self, f: &mut Frame, area: Rect) {
        let popup_area = self.centered_rect(60, 70, area);
        
        let help_text = vec![
            "Claude Flow Terminal UI - Keyboard Shortcuts",
            "",
            "Navigation:",
            "  Tab / Shift+Tab  - Switch between modes",
            "  1-9              - Jump to mode by number",
            "  q / Ctrl+C       - Quit application",
            "  F1 / ?           - Toggle this help",
            "",
            "Mode-Specific:",
            "  Dashboard        - r: refresh, space: pause",
            "  REPL            - Enter: execute, Up/Down: history",
            "  Agents          - s: spawn, k: kill, i: inspect",
            "  Memory          - /: search, Enter: view details",
            "  Swarm           - +/-: zoom, arrow keys: navigate",
            "  Logs            - f: filter, c: clear, s: save",
            "",
            "General:",
            "  Esc             - Cancel current action",
            "  Ctrl+L          - Clear screen (where applicable)",
            "  Ctrl+R          - Refresh data",
        ];

        let help_items: Vec<ListItem> = help_text
            .iter()
            .map(|&text| {
                if text.is_empty() {
                    ListItem::new(Line::from(""))
                } else if text.ends_with(':') {
                    ListItem::new(Line::from(Span::styled(
                        text,
                        Style::default().fg(self.theme.accent_color()).add_modifier(Modifier::BOLD),
                    )))
                } else {
                    ListItem::new(Line::from(text))
                }
            })
            .collect();

        let help_list = List::new(help_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .title_style(self.theme.title_style()),
            )
            .style(self.theme.normal_style());

        f.render_widget(Clear, popup_area);
        f.render_widget(help_list, popup_area);
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
    async fn handle_key_event(&mut self, key: event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::F(1) | KeyCode::Char('?') => {
                self.show_help_popup = !self.show_help_popup;
            }
            KeyCode::Esc => {
                self.show_help_popup = false;
                // Also pass to current component for canceling actions
                self.handle_component_event(key).await?;
            }
            KeyCode::Tab => {
                self.next_mode();
            }
            KeyCode::BackTab => {
                self.previous_mode();
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    let index = (digit as usize).saturating_sub(1);
                    if index < AppMode::all_modes().len() {
                        self.mode_index = index;
                        self.current_mode = AppMode::all_modes()[index].clone();
                    }
                }
            }
            KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Global refresh
                self.refresh_all_components().await?;
            }
            _ => {
                // Pass event to current component
                if !self.show_help_popup {
                    self.handle_component_event(key).await?;
                }
            }
        }
        Ok(())
    }

    #[cfg(feature = "terminal-ui")]
    async fn handle_component_event(&mut self, key: event::KeyEvent) -> Result<()> {
        match self.current_mode {
            AppMode::Dashboard => self.dashboard.handle_key(key).await?,
            AppMode::Repl => self.repl.handle_key(key).await?,
            AppMode::AgentMonitor => self.agent_monitor.handle_key(key).await?,
            AppMode::MemoryBrowser => self.memory_browser.handle_key(key).await?,
            AppMode::SwarmVisualizer => self.swarm_visualizer.handle_key(key).await?,
            AppMode::Progress => self.progress_indicators.handle_key(key).await?,
            AppMode::Logs => self.log_viewer.handle_key(key).await?,
            AppMode::Config => self.config_editor.handle_key(key).await?,
            AppMode::Help => self.help_system.handle_key(key).await?,
            AppMode::Performance => self.performance_monitor.handle_key(key).await?,
        }
        Ok(())
    }

    async fn on_tick(&mut self) -> Result<()> {
        // Update all components
        match self.current_mode {
            AppMode::Dashboard => self.dashboard.update().await?,
            AppMode::Performance => self.performance_monitor.update().await?,
            AppMode::AgentMonitor => self.agent_monitor.update().await?,
            AppMode::SwarmVisualizer => self.swarm_visualizer.update().await?,
            AppMode::Progress => self.progress_indicators.update().await?,
            AppMode::Logs => self.log_viewer.update().await?,
            _ => {} // Other components update on demand
        }
        Ok(())
    }

    fn next_mode(&mut self) {
        self.mode_index = (self.mode_index + 1) % AppMode::all_modes().len();
        self.current_mode = AppMode::all_modes()[self.mode_index].clone();
    }

    fn previous_mode(&mut self) {
        if self.mode_index == 0 {
            self.mode_index = AppMode::all_modes().len() - 1;
        } else {
            self.mode_index -= 1;
        }
        self.current_mode = AppMode::all_modes()[self.mode_index].clone();
    }

    async fn refresh_all_components(&mut self) -> Result<()> {
        info!("Refreshing all components");
        
        self.dashboard.refresh().await?;
        self.agent_monitor.refresh().await?;
        self.memory_browser.refresh().await?;
        self.swarm_visualizer.refresh().await?;
        self.progress_indicators.refresh().await?;
        self.log_viewer.refresh().await?;
        self.performance_monitor.refresh().await?;
        
        Ok(())
    }
}