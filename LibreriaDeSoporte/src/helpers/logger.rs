pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub struct Logger;

impl Logger {
    pub fn log(level: LogLevel, message: &str) {
        let prefix = match level {
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERR ]",
        };
        println!("{} {}", prefix, message);
    }
}