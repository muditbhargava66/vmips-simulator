// Copyright (c) 2024 Mudit Bhargava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// logger.rs
//
// This file contains a simple logger for the MIPS simulator.
// It provides logging to a file or to the console, with different log levels.

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
