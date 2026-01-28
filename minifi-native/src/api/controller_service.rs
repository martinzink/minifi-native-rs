use crate::{CffiLogger, DefaultLogger, LogLevel, Logger, MinifiError, MockLogger};
use crate::api::ControllerServiceContext;



pub trait ControllerService: Sized {
    fn new(logger: DefaultLogger) -> Self;
    fn log(&self, log_level: LogLevel, message: &str);
    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn disable(&mut self);  // TODO(mzink) return type?

    fn class_name() -> &'static str;
    fn group_name() -> &'static str;
    fn version() -> &'static str;
}
