use crate::c_ffi::CffiLogger;
use crate::{GetProperty, LogLevel, MinifiError};

/// This RawControllerService will be instantiated, and called on by the agent
pub trait RawControllerService: Sized {
    fn new(logger: CffiLogger) -> Self;
    fn log(&self, log_level: LogLevel, args: std::fmt::Arguments);
    fn enable<P: GetProperty>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn disable(&mut self) {}
}
