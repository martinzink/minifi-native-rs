use crate::api::ControllerServiceContext;
use crate::c_ffi::CffiLogger;
use crate::{LogLevel, MinifiError};

pub trait RawControllerService: Sized {
    fn new(logger: CffiLogger) -> Self;
    fn log(&self, log_level: LogLevel, message: &str);
    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn disable(&mut self) {}
}
