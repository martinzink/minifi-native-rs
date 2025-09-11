use crate::api::LogLevel;
use crate::api::Logger;

#[derive(Debug)]
pub struct MockLogger {
    pub logs: Vec<(LogLevel, String)>,
}

impl Logger for MockLogger {
    fn log(&mut self, level: LogLevel, message: &str) {
        self.logs.push((level, message.to_string()));
    }
}

impl MockLogger {
    pub fn new() -> Self {
        MockLogger { logs: Vec::new() }
    }
}
