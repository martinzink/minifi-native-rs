use strum_macros::{Display, EnumString, VariantNames};
use std::fmt::{Debug};

pub trait FlowFile {
}

pub trait ProcessContext {
    type FlowFile: FlowFile;

    fn get_property(
        &self,
        property_name: &str,
        flow_file: Option<&Self::FlowFile>,
    ) -> Option<String>;
}

pub trait ProcessSession {
    type FlowFile: FlowFile;

    fn create(&mut self) -> Option<Self::FlowFile>;
    fn get(&mut self) -> Option<Self::FlowFile>;
    fn transfer(&mut self, flow_file: Self::FlowFile, relationship: &str);
    fn write(&mut self, flow_file: &Self::FlowFile, data: &str);
    fn read(&mut self, flow_file: &Self::FlowFile) -> Option<String>;
}

pub trait ProcessSessionFactory {}

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