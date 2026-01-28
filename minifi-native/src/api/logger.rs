use std::fmt::Debug;

use strum_macros::{Display, EnumString, VariantNames};
#[cfg(not(test))]
use crate::{CffiLogger};

#[cfg(test)]
use crate::{MockLogger};

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
    fn log(&self, level: LogLevel, message: &str);

    fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    fn critical(&self, message: &str) {
        self.log(LogLevel::Critical, message);
    }
}


#[cfg(not(test))]
pub type DefaultLogger = CffiLogger;

#[cfg(test)]
pub type DefaultLogger = MockLogger;