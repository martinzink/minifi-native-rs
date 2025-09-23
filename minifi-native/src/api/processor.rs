use crate::api::error_code::MinifiError;
use crate::api::threading_model::{Concurrent, Exclusive, ThreadingModel};
use crate::{LogLevel, Logger, ProcessContext, ProcessSession};

pub enum ProcessorInputRequirement {
    Required,
    Allowed,
    Forbidden,
}

pub trait Processor<L: Logger>: Sized {
    type Threading: ThreadingModel;

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
    fn log(&self, log_level: LogLevel, message: &str);
    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn on_unschedule(&mut self) {}
}

pub trait ExclusiveOnTrigger<L: Logger>: Processor<L, Threading = Exclusive> {
    fn on_trigger<PC, PS>(
        &mut self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<(), MinifiError> where PC: ProcessContext, PS: ProcessSession<FlowFile = PC::FlowFile>;
}

pub trait ConcurrentOnTrigger<L: Logger>: Processor<L, Threading = Concurrent> {
    fn on_trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<(), MinifiError> where PC: ProcessContext, PS: ProcessSession<FlowFile = PC::FlowFile>;
}
