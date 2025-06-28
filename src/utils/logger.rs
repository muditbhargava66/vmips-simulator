// logger.rs
use std::fs::File;
use std::io::Write;

pub struct Logger {
    pub file: Option<File>,
    pub level: LogLevel,
}

#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl Logger {
    pub fn new(file_path: Option<&str>, level: LogLevel) -> Self {
        let file = file_path.map(|path| File::create(path).unwrap());
        Self { file, level }
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        if level as usize >= self.level as usize {
            let log_message = format!("[{:?}] {}\n", level, message);
            if let Some(file) = &mut self.file {
                file.write_all(log_message.as_bytes()).unwrap();
            } else {
                print!("{}", log_message);
            }
        }
    }

    pub fn debug(&mut self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&mut self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}
