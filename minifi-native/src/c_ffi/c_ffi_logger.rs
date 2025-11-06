use crate::api::LogLevel;
use crate::api::Logger;
use minifi_native_sys::{
    MinifiLogLevel, MinifiLogLevel_MINIFI_CRITICAL, MinifiLogLevel_MINIFI_DEBUG,
    MinifiLogLevel_MINIFI_ERROR, MinifiLogLevel_MINIFI_INFO, MinifiLogLevel_MINIFI_OFF,
    MinifiLogLevel_MINIFI_TRACE, MinifiLogLevel_MINIFI_WARNING, MinifiLogger,
    MinifiLoggerLogString, MinifiStringView,
};
use std::ffi::CString;

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
pub struct CffiLogger {
    ptr: *mut MinifiLogger,
}

impl CffiLogger {
    pub fn new(logger: *mut MinifiLogger) -> Self {
        Self { ptr: logger }
    }
}

impl Logger for CffiLogger {
    fn log(&self, level: LogLevel, message: &str) {
        if let Ok(c_message) = CString::new(message) {
            unsafe {
                MinifiLoggerLogString(
                    self.ptr,
                    level.into(),
                    MinifiStringView {
                        data: c_message.as_ptr(),
                        length: c_message.as_bytes().len(),
                    },
                );
            }
        }
    }
}
