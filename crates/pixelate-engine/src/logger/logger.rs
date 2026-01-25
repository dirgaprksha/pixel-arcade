use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Critical = 4,
}

pub struct Logger {
    min_level: AtomicU8,
}

impl Logger {
    // ANSI color codes for terminal output
    const COLOR_DEBUG: &'static str = "\x1b[90m";
    const COLOR_INFO: &'static str = "\x1b[32m";
    const COLOR_WARN: &'static str = "\x1b[33m";
    const COLOR_ERROR: &'static str = "\x1b[31m";
    const COLOR_CRITICAL: &'static str = "\x1b[1;31m";
    const COLOR_RESET: &'static str = "\x1b[0m";

    // Creates a new logger with specified minimum log level
    pub const fn new(min_level: LogLevel) -> Self {
        Self {
            min_level: AtomicU8::new(min_level as u8),
        }
    }

    // Gets current minimum log level
    pub fn min_level(&self) -> LogLevel {
        let level = self.min_level.load(Ordering::Relaxed);
        match level {
            0 => LogLevel::Debug,
            1 => LogLevel::Info,
            2 => LogLevel::Warn,
            3 => LogLevel::Error,
            _ => LogLevel::Critical,
        }
    }

    // Updates minimum log level
    pub fn set_min_level(&self, level: LogLevel) {
        self.min_level.store(level as u8, Ordering::Relaxed);
    }

    // Checks if a log message at given level should be output
    pub fn should_log(&self, level: LogLevel) -> bool {
        (level as u8) >= self.min_level.load(Ordering::Relaxed)
    }

    // Logs a message with specified level and metadata
    pub fn log(
        &self,
        level: LogLevel,
        category: &str,
        message: String,
        file: &'static str,
        line: u32,
    ) {
        if !self.should_log(level) {
            return;
        }

        let color = Self::color_level(level);
        println!(
            "{}[{:?}][{}]{} {} ({}:{})",
            color,
            level,
            category,
            Self::COLOR_RESET,
            message,
            file,
            line
        );
    }

    // Returns ANSI color code for a given log level
    fn color_level(level: LogLevel) -> &'static str {
        match level {
            LogLevel::Debug => Self::COLOR_DEBUG,
            LogLevel::Info => Self::COLOR_INFO,
            LogLevel::Warn => Self::COLOR_WARN,
            LogLevel::Error => Self::COLOR_ERROR,
            LogLevel::Critical => Self::COLOR_CRITICAL,
        }
    }
}
