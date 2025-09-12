use crate::{LogLevel, Logger, ProcessContext, ProcessSession};
use crate::api::error_code::MinifiError;
pub enum ProcessorInputRequirement {
    Required,
    Allowed,
    Forbidden,
}

/// A safe, idiomatic Rust trait for implementing a MiNiFi Processor.
pub trait Processor<L: Logger>: Sized {
    fn new(logger: L) -> Self;

    fn restore(&self) -> bool {
        false
    }

    fn get_trigger_when_empty(&self) -> bool {
        false
    }

    fn is_work_available(&self) -> bool {
        false
    }

    fn on_trigger<P: ProcessContext, S: ProcessSession>(&mut self, context: &mut P, session: &mut S) -> Result<(), MinifiError>;
    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn on_unschedule(&mut self) {}
    fn log(&self, log_level: LogLevel, message: &str);
}
