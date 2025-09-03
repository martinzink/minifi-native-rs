use std::fmt::Debug;

use strum_macros::{Display, EnumString, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames)]
#[strum(serialize_all = "PascalCase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
    Off,
}

pub trait Logger: Debug {
    fn log(&mut self, level: LogLevel, message: &str);

    fn trace(&mut self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    fn debug(&mut self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    fn warn(&mut self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    fn critical(&mut self, message: &str) {
        self.log(LogLevel::Critical, message);
    }
}
