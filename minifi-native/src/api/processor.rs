use crate::{Logger, ProcessContext, ProcessSession};

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

    fn on_trigger<P: ProcessContext, S: ProcessSession>(&mut self, context: &P, session: &mut S);
    fn on_schedule<P: ProcessContext>(
        &mut self,
        context: &P,
    );
    fn on_unschedule(&mut self) {}
}
