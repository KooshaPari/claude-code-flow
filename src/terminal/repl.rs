//! Interactive REPL component
//! Provides command execution with history and completion

use anyhow::Result;
use std::collections::VecDeque;
use tracing::{info, error};

#[cfg(feature = "terminal-ui")]
use {
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
        Frame,
    },
    tui_input::{Input, InputRequest},
};

use crate::config;

/// REPL command history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub output: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

/// REPL state
pub struct InteractiveRepl {
    config: config::UiConfig,
    input: Input,
    history: VecDeque<HistoryEntry>,
    command_history: Vec<String>,
    history_index: Option<usize>,
    max_history: usize,
    scroll_offset: usize,
    current_directory: std::path::PathBuf,
    prompt: String,
    completions: Vec<String>,
    show_completions: bool,
}

impl InteractiveRepl {
    pub async fn new(config: &config::UiConfig) -> Result<Self> {
        info!("Initializing interactive REPL");
        
        let current_directory = std::env::current_dir().unwrap_or_else(|_| "/".into());
        let prompt = format!("claude-flow:{}> ", current_directory.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?"));
        
        Ok(Self {
            config: config.clone(),
            input: Input::default(),
            history: VecDeque::new(),
            command_history: Vec::new(),
            history_index: None,
            max_history: 1000,
            scroll_offset: 0,
            current_directory,
            prompt,
            completions: Self::get_builtin_commands(),
            show_completions: false,
        })
    }

    /// Run the REPL in standalone mode
    pub async fn run(&self) -> Result<()> {
        info!("Starting REPL in standalone mode");
        
        #[cfg(feature = "terminal-ui")]
        {
            // This would typically set up a simpler terminal interface
            // For now, we'll just print a message
            println!("REPL mode - use the main terminal app for full interface");
        }
        
        Ok(())
    }

    #[cfg(feature = "terminal-ui")]
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),     // History area
                Constraint::Length(3),  // Input area
                Constraint::Length(if self.show_completions { 5 } else { 0 }), // Completions
            ])
            .split(area);

        self.draw_history(f, chunks[0]);
        self.draw_input(f, chunks[1]);
        
        if self.show_completions && chunks.len() > 2 {
            self.draw_completions(f, chunks[2]);
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_history(&self, f: &mut Frame, area: Rect) {
        let history_items: Vec<ListItem> = self.history
            .iter()
            .skip(self.scroll_offset)
            .flat_map(|entry| {
                let timestamp = entry.timestamp.format("%H:%M:%S");
                let status_style = if entry.success {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                };
                
                let mut items = vec![
                    ListItem::new(Line::from(vec![
                        Span::styled(format!("[{}] ", timestamp), Style::default().fg(Color::Gray)),
                        Span::styled(&self.prompt, Style::default().fg(Color::Cyan)),
                        Span::styled(&entry.command, Style::default().fg(Color::White)),
                    ])),
                ];
                
                if !entry.output.is_empty() {
                    for line in entry.output.lines() {
                        items.push(ListItem::new(Line::from(
                            Span::styled(format!("  {}", line), status_style)
                        )));
                    }
                }
                
                items.push(ListItem::new(Line::from(""))); // Empty line separator
                items
            })
            .collect();

        let history_list = List::new(history_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Command History")
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default());

        f.render_widget(history_list, area);

        // Render scrollbar if needed
        if self.history.len() > area.height as usize {
            let mut scrollbar_state = ScrollbarState::default()
                .content_length(self.history.len())
                .position(self.scroll_offset);
            
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));
            
            f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
        }
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_input(&self, f: &mut Frame, area: Rect) {
        let input_paragraph = Paragraph::new(self.input.value())
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Input - {}", self.prompt))
                    .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            );

        f.render_widget(input_paragraph, area);

        // Set cursor position
        f.set_cursor_position(ratatui::layout::Position { 
            x: area.x + self.input.visual_cursor() as u16 + 1,
            y: area.y + 1,
        });
    }

    #[cfg(feature = "terminal-ui")]
    fn draw_completions(&self, f: &mut Frame, area: Rect) {
        if self.completions.is_empty() {
            return;
        }

        let completion_items: Vec<ListItem> = self.completions
            .iter()
            .take(5) // Show max 5 completions
            .map(|completion| {
                ListItem::new(Line::from(Span::styled(
                    completion,
                    Style::default().fg(Color::Yellow),
                )))
            })
            .collect();

        let completions_list = List::new(completion_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Completions")
                    .title_style(Style::default().fg(Color::Yellow)),
            );

        f.render_widget(completions_list, area);
    }

    #[cfg(feature = "terminal-ui")]
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                let command = self.input.value().to_string();
                self.execute_command(command).await?;
                self.input.reset();
                self.show_completions = false;
            }
            KeyCode::Up => {
                self.navigate_history_up();
            }
            KeyCode::Down => {
                self.navigate_history_down();
            }
            KeyCode::Tab => {
                self.show_completions = !self.show_completions;
                if self.show_completions {
                    self.update_completions();
                }
            }
            KeyCode::PageUp => {
                if self.scroll_offset > 0 {
                    self.scroll_offset = self.scroll_offset.saturating_sub(10);
                }
            }
            KeyCode::PageDown => {
                if self.scroll_offset + 10 < self.history.len() {
                    self.scroll_offset += 10;
                }
            }
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.clear_history();
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.input.reset();
                self.show_completions = false;
            }
            _ => {
                // Handle normal input
                let request = InputRequest::from(key);
                if let Some(req) = request {
                    self.input.handle(req);
                    self.update_completions();
                }
            }
        }
        Ok(())
    }

    async fn execute_command(&mut self, command: String) -> Result<()> {
        if command.trim().is_empty() {
            return Ok(());
        }

        info!("Executing command: {}", command);
        
        // Add to command history
        self.command_history.push(command.clone());
        if self.command_history.len() > self.max_history {
            self.command_history.remove(0);
        }
        self.history_index = None;

        let start_time = chrono::Utc::now();
        let (output, success) = self.run_command(&command).await;

        let entry = HistoryEntry {
            command,
            output,
            timestamp: start_time,
            success,
        };

        self.history.push_back(entry);
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }

        // Auto-scroll to bottom
        self.scroll_offset = self.history.len().saturating_sub(10);

        Ok(())
    }

    async fn run_command(&mut self, command: &str) -> (String, bool) {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return (String::new(), true);
        }

        match parts[0] {
            "help" | "?" => {
                (self.get_help_text(), true)
            }
            "clear" | "cls" => {
                self.clear_history();
                ("Screen cleared".to_string(), true)
            }
            "history" => {
                let history_text = self.command_history
                    .iter()
                    .enumerate()
                    .map(|(i, cmd)| format!("{:3}: {}", i + 1, cmd))
                    .collect::<Vec<_>>()
                    .join("\\n");
                (history_text, true)
            }
            "pwd" => {
                (self.current_directory.display().to_string(), true)
            }
            "cd" => {
                if parts.len() < 2 {
                    (self.current_directory.display().to_string(), true)
                } else {
                    self.change_directory(parts[1])
                }
            }
            "ls" | "dir" => {
                self.list_directory(parts.get(1).copied()).await
            }
            "echo" => {
                let text = parts[1..].join(" ");
                (text, true)
            }
            "status" => {
                self.get_system_status().await
            }
            "agents" => {
                self.handle_agent_command(&parts[1..]).await
            }
            "memory" => {
                self.handle_memory_command(&parts[1..]).await
            }
            "swarm" => {
                self.handle_swarm_command(&parts[1..]).await
            }
            "config" => {
                self.handle_config_command(&parts[1..]).await
            }
            _ => {
                // Try to execute as external command
                self.execute_external_command(command).await
            }
        }
    }

    fn change_directory(&mut self, path: &str) -> (String, bool) {
        let new_path: std::path::PathBuf = if path == "~" {
            std::env::var("HOME").unwrap_or_else(|_| "/".to_string()).into()
        } else if path.starts_with("~/") {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
            format!("{}/{}", home, &path[2..]).into()
        } else {
            path.into()
        };

        match std::env::set_current_dir(&new_path) {
            Ok(_) => {
                self.current_directory = std::env::current_dir().unwrap_or_else(|_| "/".into());
                self.update_prompt();
                (format!("Changed to: {}", self.current_directory.display()), true)
            }
            Err(e) => {
                (format!("Error: {}", e), false)
            }
        }
    }

    async fn list_directory(&self, path: Option<&str>) -> (String, bool) {
        let target_path = path.map(|p| std::path::Path::new(p)).unwrap_or(&self.current_directory);
        
        match std::fs::read_dir(target_path) {
            Ok(entries) => {
                let mut items = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let metadata = entry.metadata();
                        let file_type = if metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false) {
                            "DIR "
                        } else {
                            "FILE"
                        };
                        let size = metadata.as_ref()
                            .map(|m| m.len())
                            .unwrap_or(0);
                        items.push(format!("{} {:>10} {}", file_type, size, name));
                    }
                }
                items.sort();
                (items.join("\\n"), true)
            }
            Err(e) => {
                (format!("Error listing directory: {}", e), false)
            }
        }
    }

    async fn get_system_status(&self) -> (String, bool) {
        let status = format!(
            "Claude Flow System Status\\n\
             Current Directory: {}\\n\
             Commands in History: {}\\n\
             Memory Usage: {} MB\\n\
             Terminal Size: ?x?\\n\
             Uptime: ?",
            self.current_directory.display(),
            self.command_history.len(),
            // This would get real memory usage
            (std::mem::size_of_val(self) / 1024 / 1024)
        );
        (status, true)
    }

    async fn handle_agent_command(&self, args: &[&str]) -> (String, bool) {
        if args.is_empty() {
            return ("Usage: agents [list|spawn|kill|info] [args...]".to_string(), false);
        }

        match args[0] {
            "list" => ("No agents currently running".to_string(), true),
            "spawn" => ("Agent spawning not implemented in REPL".to_string(), false),
            "kill" => ("Agent termination not implemented in REPL".to_string(), false),
            "info" => ("Agent info not implemented in REPL".to_string(), false),
            _ => (format!("Unknown agent command: {}", args[0]), false),
        }
    }

    async fn handle_memory_command(&self, args: &[&str]) -> (String, bool) {
        if args.is_empty() {
            return ("Usage: memory [stats|query|clear] [args...]".to_string(), false);
        }

        match args[0] {
            "stats" => ("Memory stats not implemented in REPL".to_string(), false),
            "query" => ("Memory query not implemented in REPL".to_string(), false),
            "clear" => ("Memory clear not implemented in REPL".to_string(), false),
            _ => (format!("Unknown memory command: {}", args[0]), false),
        }
    }

    async fn handle_swarm_command(&self, args: &[&str]) -> (String, bool) {
        if args.is_empty() {
            return ("Usage: swarm [status|init|stop] [args...]".to_string(), false);
        }

        match args[0] {
            "status" => ("No active swarm".to_string(), true),
            "init" => ("Swarm initialization not implemented in REPL".to_string(), false),
            "stop" => ("Swarm termination not implemented in REPL".to_string(), false),
            _ => (format!("Unknown swarm command: {}", args[0]), false),
        }
    }

    async fn handle_config_command(&self, args: &[&str]) -> (String, bool) {
        if args.is_empty() {
            return ("Usage: config [show|set|reset] [args...]".to_string(), false);
        }

        match args[0] {
            "show" => {
                let config_str = format!("Config:\\n{:#?}", self.config);
                (config_str, true)
            }
            "set" => ("Config modification not implemented in REPL".to_string(), false),
            "reset" => ("Config reset not implemented in REPL".to_string(), false),
            _ => (format!("Unknown config command: {}", args[0]), false),
        }
    }

    async fn execute_external_command(&self, command: &str) -> (String, bool) {
        // This would execute external commands via shell
        // For now, return a placeholder
        (format!("External command execution not implemented: {}", command), false)
    }

    fn navigate_history_up(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        let new_index = match self.history_index {
            None => Some(self.command_history.len() - 1),
            Some(idx) if idx > 0 => Some(idx - 1),
            Some(_) => Some(0),
        };

        if let Some(idx) = new_index {
            self.history_index = Some(idx);
            let command = self.command_history[idx].clone();
            self.input = Input::from(command);
        }
    }

    fn navigate_history_down(&mut self) {
        if let Some(idx) = self.history_index {
            if idx + 1 < self.command_history.len() {
                self.history_index = Some(idx + 1);
                let command = self.command_history[idx + 1].clone();
                self.input = Input::from(command);
            } else {
                self.history_index = None;
                self.input.reset();
            }
        }
    }

    fn update_completions(&mut self) {
        let input_value = self.input.value();
        if input_value.is_empty() {
            self.completions = Self::get_builtin_commands();
            return;
        }

        let parts: Vec<&str> = input_value.split_whitespace().collect();
        if parts.len() <= 1 {
            // Complete command names
            self.completions = Self::get_builtin_commands()
                .into_iter()
                .filter(|cmd| cmd.starts_with(input_value))
                .collect();
        } else {
            // Complete subcommands or file paths
            self.completions = Vec::new(); // TODO: Implement sophisticated completion
        }
    }

    fn get_builtin_commands() -> Vec<String> {
        vec![
            "help".to_string(),
            "clear".to_string(),
            "history".to_string(),
            "pwd".to_string(),
            "cd".to_string(),
            "ls".to_string(),
            "echo".to_string(),
            "status".to_string(),
            "agents".to_string(),
            "memory".to_string(),
            "swarm".to_string(),
            "config".to_string(),
        ]
    }

    fn get_help_text(&self) -> String {
        "Claude Flow Interactive REPL\\n\
         \\n\
         Built-in Commands:\\n\
         help, ?          - Show this help\\n\
         clear, cls       - Clear the screen\\n\
         history          - Show command history\\n\
         pwd              - Show current directory\\n\
         cd <path>        - Change directory\\n\
         ls [path]        - List directory contents\\n\
         echo <text>      - Echo text\\n\
         status           - Show system status\\n\
         \\n\
         Claude Flow Commands:\\n\
         agents <cmd>     - Agent management\\n\
         memory <cmd>     - Memory operations\\n\
         swarm <cmd>      - Swarm operations\\n\
         config <cmd>     - Configuration\\n\
         \\n\
         Navigation:\\n\
         Up/Down arrows   - Navigate command history\\n\
         Tab              - Toggle completions\\n\
         Ctrl+L           - Clear screen\\n\
         Ctrl+C           - Cancel input\\n\
         Enter            - Execute command".to_string()
    }

    fn clear_history(&mut self) {
        self.history.clear();
        self.scroll_offset = 0;
    }

    fn update_prompt(&mut self) {
        self.prompt = format!("claude-flow:{}> ", 
            self.current_directory.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?"));
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update REPL state if needed
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        // Refresh REPL state
        self.update_prompt();
        Ok(())
    }
}