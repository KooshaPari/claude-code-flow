//! Event handling system for terminal UI
//! Manages keyboard, mouse, and system events

use anyhow::Result;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tracing::{info, error};

#[cfg(feature = "terminal-ui")]
use crossterm::event::{self, Event, KeyEvent, MouseEvent};

/// Terminal event types
#[derive(Debug, Clone)]
pub enum TerminalEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
    Quit,
}

/// Event handler for terminal UI
pub struct EventHandler {
    #[cfg(feature = "terminal-ui")]
    receiver: mpsc::Receiver<TerminalEvent>,
    #[cfg(feature = "terminal-ui")]
    _handle: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        info!("Initializing event handler");
        
        #[cfg(feature = "terminal-ui")]
        {
            let (sender, receiver) = mpsc::channel();
            let tick_rate = Duration::from_millis(250);
            
            let handle = thread::spawn(move || {
                let mut last_tick = std::time::Instant::now();
                
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));
                    
                    if event::poll(timeout).unwrap_or(false) {
                        match event::read() {
                            Ok(Event::Key(key)) => {
                                if sender.send(TerminalEvent::Key(key)).is_err() {
                                    break;
                                }
                            }
                            Ok(Event::Mouse(mouse)) => {
                                if sender.send(TerminalEvent::Mouse(mouse)).is_err() {
                                    break;
                                }
                            }
                            Ok(Event::Resize(width, height)) => {
                                if sender.send(TerminalEvent::Resize(width, height)).is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                error!("Error reading terminal event: {}", e);
                            }
                            _ => {}
                        }
                    }
                    
                    if last_tick.elapsed() >= tick_rate {
                        if sender.send(TerminalEvent::Tick).is_err() {
                            break;
                        }
                        last_tick = std::time::Instant::now();
                    }
                }
            });
            
            Self {
                receiver,
                _handle: handle,
            }
        }
        
        #[cfg(not(feature = "terminal-ui"))]
        {
            Self {}
        }
    }

    /// Try to receive the next event without blocking
    #[cfg(feature = "terminal-ui")]
    pub fn try_recv(&self) -> Result<Option<TerminalEvent>, mpsc::TryRecvError> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn try_recv(&self) -> Result<Option<TerminalEvent>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    /// Receive the next event, blocking until one is available
    #[cfg(feature = "terminal-ui")]
    pub fn recv(&self) -> Result<TerminalEvent, mpsc::RecvError> {
        self.receiver.recv()
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn recv(&self) -> Result<TerminalEvent, Box<dyn std::error::Error + Send + Sync>> {
        std::thread::sleep(Duration::from_millis(100));
        Ok(TerminalEvent::Tick)
    }

    /// Receive the next event with a timeout
    #[cfg(feature = "terminal-ui")]
    pub fn recv_timeout(&self, timeout: Duration) -> Result<TerminalEvent, mpsc::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn recv_timeout(&self, timeout: Duration) -> Result<TerminalEvent, Box<dyn std::error::Error + Send + Sync>> {
        std::thread::sleep(timeout);
        Ok(TerminalEvent::Tick)
    }
}

/// Event processing utilities
pub struct EventProcessor;

impl EventProcessor {
    /// Check if a key event should quit the application
    #[cfg(feature = "terminal-ui")]
    pub fn is_quit_event(event: &TerminalEvent) -> bool {
        matches!(
            event,
            TerminalEvent::Key(KeyEvent {
                code: crossterm::event::KeyCode::Char('q'),
                modifiers: crossterm::event::KeyModifiers::NONE,
                ..
            }) | TerminalEvent::Key(KeyEvent {
                code: crossterm::event::KeyCode::Char('c'),
                modifiers: crossterm::event::KeyModifiers::CONTROL,
                ..
            }) | TerminalEvent::Quit
        )
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn is_quit_event(_event: &TerminalEvent) -> bool {
        false
    }

    /// Check if a key event is a help request
    #[cfg(feature = "terminal-ui")]
    pub fn is_help_event(event: &TerminalEvent) -> bool {
        matches!(
            event,
            TerminalEvent::Key(KeyEvent {
                code: crossterm::event::KeyCode::F(1),
                ..
            }) | TerminalEvent::Key(KeyEvent {
                code: crossterm::event::KeyCode::Char('?'),
                modifiers: crossterm::event::KeyModifiers::NONE,
                ..
            })
        )
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn is_help_event(_event: &TerminalEvent) -> bool {
        false
    }

    /// Check if a key event is a refresh request
    #[cfg(feature = "terminal-ui")]
    pub fn is_refresh_event(event: &TerminalEvent) -> bool {
        matches!(
            event,
            TerminalEvent::Key(KeyEvent {
                code: crossterm::event::KeyCode::Char('r'),
                modifiers: crossterm::event::KeyModifiers::CONTROL,
                ..
            }) | TerminalEvent::Key(KeyEvent {
                code: crossterm::event::KeyCode::F(5),
                ..
            })
        )
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn is_refresh_event(_event: &TerminalEvent) -> bool {
        false
    }

    /// Extract key code from a key event
    #[cfg(feature = "terminal-ui")]
    pub fn extract_key_code(event: &TerminalEvent) -> Option<crossterm::event::KeyCode> {
        match event {
            TerminalEvent::Key(key_event) => Some(key_event.code),
            _ => None,
        }
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn extract_key_code(_event: &TerminalEvent) -> Option<char> {
        None
    }

    /// Check if event is a navigation event (arrow keys, tab, etc.)
    #[cfg(feature = "terminal-ui")]
    pub fn is_navigation_event(event: &TerminalEvent) -> bool {
        matches!(
            event,
            TerminalEvent::Key(KeyEvent {
                code: crossterm::event::KeyCode::Up
                    | crossterm::event::KeyCode::Down
                    | crossterm::event::KeyCode::Left
                    | crossterm::event::KeyCode::Right
                    | crossterm::event::KeyCode::Tab
                    | crossterm::event::KeyCode::BackTab
                    | crossterm::event::KeyCode::Home
                    | crossterm::event::KeyCode::End
                    | crossterm::event::KeyCode::PageUp
                    | crossterm::event::KeyCode::PageDown,
                ..
            })
        )
    }

    #[cfg(not(feature = "terminal-ui"))]
    pub fn is_navigation_event(_event: &TerminalEvent) -> bool {
        false
    }
}

/// Event rate limiter to prevent excessive processing
pub struct EventRateLimiter {
    last_event_time: std::time::Instant,
    min_interval: Duration,
}

impl EventRateLimiter {
    pub fn new(min_interval: Duration) -> Self {
        Self {
            last_event_time: std::time::Instant::now(),
            min_interval,
        }
    }

    /// Check if enough time has passed since the last event
    pub fn should_process(&mut self) -> bool {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_event_time) >= self.min_interval {
            self.last_event_time = now;
            true
        } else {
            false
        }
    }

    /// Reset the rate limiter
    pub fn reset(&mut self) {
        self.last_event_time = std::time::Instant::now();
    }
}

/// Event statistics for monitoring
#[derive(Debug, Default)]
pub struct EventStats {
    pub total_events: u64,
    pub key_events: u64,
    pub mouse_events: u64,
    pub resize_events: u64,
    pub tick_events: u64,
    pub dropped_events: u64,
}

impl EventStats {
    pub fn record_event(&mut self, event: &TerminalEvent) {
        self.total_events += 1;
        
        match event {
            TerminalEvent::Key(_) => self.key_events += 1,
            TerminalEvent::Mouse(_) => self.mouse_events += 1,
            TerminalEvent::Resize(_, _) => self.resize_events += 1,
            TerminalEvent::Tick => self.tick_events += 1,
            TerminalEvent::Quit => {}
        }
    }

    pub fn record_dropped_event(&mut self) {
        self.dropped_events += 1;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn events_per_second(&self, duration: Duration) -> f64 {
        if duration.as_secs_f64() > 0.0 {
            self.total_events as f64 / duration.as_secs_f64()
        } else {
            0.0
        }
    }
}