use crate::api::LogLevel;
use crate::api::Logger;
use std::sync::Mutex;

#[derive(Debug)]
pub struct MockLogger {
    pub logs: Mutex<Vec<(LogLevel, String)>>,
}

impl Logger for MockLogger {
    fn log(&self, level: LogLevel, message: &str) {
        let mut logs_guard = self.logs.lock().unwrap();
        logs_guard.push((level, message.to_string()));
    }
}

impl MockLogger {
    pub fn new() -> Self {
        MockLogger {
            logs: Mutex::new(Vec::new()),
        }
    }
}
