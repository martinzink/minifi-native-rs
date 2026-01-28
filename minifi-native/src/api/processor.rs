use crate::api::error_code::MinifiError;
use crate::api::threading_model::{Concurrent, Exclusive, ThreadingModel};
use crate::{DefaultLogger, LogLevel, ProcessContext, ProcessSession};

pub enum ProcessorInputRequirement {
    Required,
    Allowed,
    Forbidden,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OnTriggerResult {
    Ok,
    Yield,
}

pub trait Processor: Sized {
    type Threading: ThreadingModel;

    fn new(logger: DefaultLogger) -> Self;
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
    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        vec![]
    }
}

pub trait ExclusiveOnTrigger: Processor<Threading = Exclusive> {
    fn on_trigger<PC, PS>(
        &mut self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>;
}

pub trait ConcurrentOnTrigger: Processor<Threading = Concurrent> {
    fn on_trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>;
}
