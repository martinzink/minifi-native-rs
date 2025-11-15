use crate::api::LogLevel;
use crate::api::Logger;
use minifi_native_sys::{
    MinifiLogLevel, MinifiLogger,
    MinifiLoggerLogString, MinifiStringView,
};
use std::ffi::CString;

impl From<LogLevel> for MinifiLogLevel {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => minifi_native_sys::MinifiLogLevel_MINIFI_LOG_LEVEL_TRACE,
            LogLevel::Debug => minifi_native_sys::MinifiLogLevel_MINIFI_LOG_LEVEL_DEBUG,
            LogLevel::Info => minifi_native_sys::MinifiLogLevel_MINIFI_LOG_LEVEL_INFO,
            LogLevel::Warn => minifi_native_sys::MinifiLogLevel_MINIFI_LOG_LEVEL_WARNING,
            LogLevel::Error => minifi_native_sys::MinifiLogLevel_MINIFI_LOG_LEVEL_ERROR,
            LogLevel::Critical => minifi_native_sys::MinifiLogLevel_MINIFI_LOG_LEVEL_CRITICAL,
            LogLevel::Off => minifi_native_sys::MinifiLogLevel_MINIFI_LOG_LEVEL_OFF,
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
