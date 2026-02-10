use crate::api::ControllerServiceContext;
use crate::{DefaultLogger, LogLevel, MinifiError, Property};

pub trait ControllerService: Sized {
    fn new(logger: DefaultLogger) -> Self;
    fn log(&self, log_level: LogLevel, message: &str);
    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn disable(&mut self) {}
}

pub trait ControllerServiceDefinition {
    const DESCRIPTION: &'static str;
    const PROPERTIES: &'static [Property];
}
