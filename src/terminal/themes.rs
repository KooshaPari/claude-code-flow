//! Theme system for terminal UI
//! Manages colors, styles, and visual appearance

#[cfg(feature = "terminal-ui")]
use ratatui::style::{Color, Modifier, Style};

/// Color scheme configuration
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub foreground: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub muted: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Yellow,
            background: Color::Black,
            foreground: Color::White,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            muted: Color::Gray,
        }
    }
}

impl ColorScheme {
    /// Dark theme color scheme
    pub fn dark() -> Self {
        Self {
            primary: Color::Rgb(100, 181, 246),      // Light blue
            secondary: Color::Rgb(69, 90, 100),      // Blue gray
            accent: Color::Rgb(255, 193, 7),         // Amber
            background: Color::Rgb(18, 18, 18),      // Very dark gray
            foreground: Color::Rgb(255, 255, 255),   // White
            success: Color::Rgb(76, 175, 80),        // Green
            warning: Color::Rgb(255, 152, 0),        // Orange
            error: Color::Rgb(244, 67, 54),          // Red
            info: Color::Rgb(33, 150, 243),          // Blue
            muted: Color::Rgb(158, 158, 158),        // Gray
        }
    }

    /// Light theme color scheme
    pub fn light() -> Self {
        Self {
            primary: Color::Rgb(25, 118, 210),       // Blue
            secondary: Color::Rgb(96, 125, 139),     // Blue gray
            accent: Color::Rgb(245, 124, 0),         // Orange
            background: Color::Rgb(250, 250, 250),   // Light gray
            foreground: Color::Rgb(33, 33, 33),      // Dark gray
            success: Color::Rgb(56, 142, 60),        // Green
            warning: Color::Rgb(239, 108, 0),        // Orange
            error: Color::Rgb(211, 47, 47),          // Red
            info: Color::Rgb(25, 118, 210),          // Blue
            muted: Color::Rgb(117, 117, 117),        // Gray
        }
    }

    /// High contrast theme for accessibility
    pub fn high_contrast() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::Yellow,
            accent: Color::Cyan,
            background: Color::Black,
            foreground: Color::White,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            muted: Color::White,
        }
    }

    /// Retro/vintage theme
    pub fn retro() -> Self {
        Self {
            primary: Color::Rgb(0, 255, 0),          // Bright green
            secondary: Color::Rgb(0, 200, 0),        // Green
            accent: Color::Rgb(255, 255, 0),         // Yellow
            background: Color::Black,
            foreground: Color::Rgb(0, 255, 0),       // Bright green
            success: Color::Rgb(0, 255, 0),          // Green
            warning: Color::Rgb(255, 255, 0),        // Yellow
            error: Color::Rgb(255, 0, 0),            // Red
            info: Color::Rgb(0, 255, 255),           // Cyan
            muted: Color::Rgb(0, 128, 0),            // Dark green
        }
    }
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ColorScheme,
    pub use_bold: bool,
    pub use_italic: bool,
    pub use_underline: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            colors: ColorScheme::default(),
            use_bold: true,
            use_italic: false,
            use_underline: false,
        }
    }
}

impl Theme {
    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            colors: ColorScheme::dark(),
            use_bold: true,
            use_italic: false,
            use_underline: false,
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            colors: ColorScheme::light(),
            use_bold: true,
            use_italic: false,
            use_underline: false,
        }
    }

    /// Create a high contrast theme
    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            colors: ColorScheme::high_contrast(),
            use_bold: true,
            use_italic: false,
            use_underline: true,
        }
    }

    /// Create a retro theme
    pub fn retro() -> Self {
        Self {
            name: "Retro".to_string(),
            colors: ColorScheme::retro(),
            use_bold: true,
            use_italic: false,
            use_underline: false,
        }
    }

    /// Get all available themes
    pub fn all_themes() -> Vec<Theme> {
        vec![
            Theme::default(),
            Theme::dark(),
            Theme::light(),
            Theme::high_contrast(),
            Theme::retro(),
        ]
    }

    // Style methods for consistent styling across the application
    #[cfg(feature = "terminal-ui")]
    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.colors.foreground)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn title_style(&self) -> Style {
        let mut style = Style::default().fg(self.colors.primary);
        if self.use_bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        style
    }

    #[cfg(feature = "terminal-ui")]
    pub fn accent_style(&self) -> Style {
        Style::default().fg(self.colors.accent)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn success_style(&self) -> Style {
        Style::default().fg(self.colors.success)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.colors.warning)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.colors.error)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn info_style(&self) -> Style {
        Style::default().fg(self.colors.info)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.colors.muted)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.colors.background)
            .bg(self.colors.primary)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn tab_style(&self) -> Style {
        Style::default().fg(self.colors.muted)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn tab_selected_style(&self) -> Style {
        let mut style = Style::default().fg(self.colors.accent);
        if self.use_bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        if self.use_underline {
            style = style.add_modifier(Modifier::UNDERLINED);
        }
        style
    }

    #[cfg(feature = "terminal-ui")]
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.colors.primary)
    }

    #[cfg(feature = "terminal-ui")]
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.colors.foreground)
            .bg(self.colors.secondary)
    }

    // Color accessors for components that need direct color access
    pub fn primary_color(&self) -> Color {
        self.colors.primary
    }

    pub fn secondary_color(&self) -> Color {
        self.colors.secondary
    }

    pub fn accent_color(&self) -> Color {
        self.colors.accent
    }

    pub fn background_color(&self) -> Color {
        self.colors.background
    }

    pub fn foreground_color(&self) -> Color {
        self.colors.foreground
    }

    pub fn success_color(&self) -> Color {
        self.colors.success
    }

    pub fn warning_color(&self) -> Color {
        self.colors.warning
    }

    pub fn error_color(&self) -> Color {
        self.colors.error
    }

    pub fn info_color(&self) -> Color {
        self.colors.info
    }

    pub fn muted_color(&self) -> Color {
        self.colors.muted
    }
}

/// Theme manager for handling theme switching and persistence
pub struct ThemeManager {
    current_theme: Theme,
    available_themes: Vec<Theme>,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            current_theme: Theme::default(),
            available_themes: Theme::all_themes(),
        }
    }
}

impl ThemeManager {
    /// Create a new theme manager with the default theme
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a theme manager with a specific theme
    pub fn with_theme(theme: Theme) -> Self {
        Self {
            current_theme: theme,
            available_themes: Theme::all_themes(),
        }
    }

    /// Get the current theme
    pub fn current_theme(&self) -> &Theme {
        &self.current_theme
    }

    /// Set the current theme by name
    pub fn set_theme_by_name(&mut self, name: &str) -> Result<(), String> {
        if let Some(theme) = self.available_themes.iter().find(|t| t.name == name) {
            self.current_theme = theme.clone();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    /// Set the current theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.current_theme = theme;
    }

    /// Get available theme names
    pub fn available_theme_names(&self) -> Vec<&str> {
        self.available_themes.iter().map(|t| t.name.as_str()).collect()
    }

    /// Cycle to the next theme
    pub fn next_theme(&mut self) {
        let current_index = self.available_themes
            .iter()
            .position(|t| t.name == self.current_theme.name)
            .unwrap_or(0);
        
        let next_index = (current_index + 1) % self.available_themes.len();
        self.current_theme = self.available_themes[next_index].clone();
    }

    /// Cycle to the previous theme
    pub fn previous_theme(&mut self) {
        let current_index = self.available_themes
            .iter()
            .position(|t| t.name == self.current_theme.name)
            .unwrap_or(0);
        
        let prev_index = if current_index == 0 {
            self.available_themes.len() - 1
        } else {
            current_index - 1
        };
        
        self.current_theme = self.available_themes[prev_index].clone();
    }

    /// Load theme from configuration
    pub fn load_from_config(&mut self, theme_name: Option<&str>) {
        if let Some(name) = theme_name {
            let _ = self.set_theme_by_name(name);
        }
    }

    /// Save current theme preference (placeholder for actual persistence)
    pub fn save_preference(&self) -> Result<(), std::io::Error> {
        // In a real implementation, this would save to a config file
        tracing::info!("Theme preference saved: {}", self.current_theme.name);
        Ok(())
    }
}