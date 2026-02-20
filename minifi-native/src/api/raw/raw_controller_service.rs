use crate::api::ControllerServiceContext;
use crate::{LogLevel, MinifiError};
use crate::c_ffi::CffiLogger;

pub trait RawControllerService: Sized {
    fn new(logger: CffiLogger) -> Self;
    fn log(&self, log_level: LogLevel, message: &str);
    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn disable(&mut self) {}
}
