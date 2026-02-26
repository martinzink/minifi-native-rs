use crate::c_ffi::CffiLogger;
use crate::{GetProperty, LogLevel, MinifiError};

pub trait RawControllerService: Sized {
    fn new(logger: CffiLogger) -> Self;
    fn log(&self, log_level: LogLevel, message: &str);
    fn enable<P: GetProperty>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn disable(&mut self) {}
}
