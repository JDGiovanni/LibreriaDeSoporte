use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::Mutex;
use std::path::PathBuf;
use chrono::Local;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERR ]",
        }
    }
}

pub struct Logger {
    file: Mutex<Option<File>>,
    log_to_console: bool,
    log_to_file: bool,
}

impl Logger {
    /// Obtiene el timestamp actual en hora LOCAL (formato HH:MM:SS 24 horas)
    fn get_timestamp() -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    pub fn new() -> Self {
        Logger {
            file: Mutex::new(None),
            log_to_console: true,
            log_to_file: false,
        }
    }

    pub fn with_file(path: PathBuf) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        Ok(Logger {
            file: Mutex::new(Some(file)),
            log_to_console: true,
            log_to_file: true,
        })
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let timestamp = Self::get_timestamp();
        let prefix = level.as_str();
        let log_line = format!("[{}] {} {}", timestamp, prefix, message);
        
        if self.log_to_console {
            self.write_to_console(&log_line, &level);
        }
        
        if self.log_to_file {
            self.write_to_file(&log_line);
        }
    }

    fn write_to_console(&self, log_line: &str, level: &LogLevel) {
        match level {
            LogLevel::Info => println!("{}", log_line),
            LogLevel::Warning => println!("\x1b[33m{}\x1b[0m", log_line),
            LogLevel::Error => println!("\x1b[31m{}\x1b[0m", log_line),
        }
    }

    fn write_to_file(&self, log_line: &str) {
        let mut file_guard = self.file.lock().unwrap();
        if let Some(file) = file_guard.as_mut() {
            let _ = writeln!(file, "{}", log_line);
        }
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
