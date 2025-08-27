use minifi_native_sys::{
    MinifiLogLevel, MinifiLogLevel_MINIFI_CRITICAL, MinifiLogLevel_MINIFI_DEBUG,
    MinifiLogLevel_MINIFI_ERROR, MinifiLogLevel_MINIFI_INFO, MinifiLogLevel_MINIFI_OFF,
    MinifiLogLevel_MINIFI_TRACE, MinifiLogLevel_MINIFI_WARNING, MinifiLogger,
    MinifiLoggerLogString, MinifiStringView,
};
use std::ffi::CString;
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

impl From<LogLevel> for MinifiLogLevel {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => MinifiLogLevel_MINIFI_TRACE,
            LogLevel::Debug => MinifiLogLevel_MINIFI_DEBUG,
            LogLevel::Info => MinifiLogLevel_MINIFI_INFO,
            LogLevel::Warn => MinifiLogLevel_MINIFI_WARNING,
            LogLevel::Error => MinifiLogLevel_MINIFI_ERROR,
            LogLevel::Critical => MinifiLogLevel_MINIFI_CRITICAL,
            LogLevel::Off => MinifiLogLevel_MINIFI_OFF,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Logger(MinifiLogger);

impl Logger {
    pub fn new(logger: MinifiLogger) -> Self {
        Self(logger)
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if let Ok(c_message) = CString::new(message) {
            unsafe {
                MinifiLoggerLogString(
                    self.0,
                    level.into(),
                    MinifiStringView {
                        data: c_message.as_ptr(),
                        length: c_message.as_bytes().len() as u32,
                    },
                );
            }
        }
    }

    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    pub fn critical(&self, message: &str) {
        self.log(LogLevel::Critical, message);
    }
}
