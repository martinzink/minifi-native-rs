use crate::api::errors::MinifiError;
use crate::api::threading_model::{Concurrent, Exclusive, ThreadingModel};
use crate::{DefaultLogger, DynProcessorDefinition, LogLevel, ProcessContext, ProcessSession};

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

pub trait RawProcessor: Sized {
    type Threading: ThreadingModel;

    fn new(logger: DefaultLogger) -> Self;
    fn restore(&self) -> bool {
        false
    } // TODO(mzink)
    fn get_trigger_when_empty(&self) -> bool {
        false
    } // TODO(mzink)
    fn is_work_available(&self) -> bool {
        false
    } // TODO(mzink)
    fn log(&self, log_level: LogLevel, message: &str);
    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError>;
    fn on_unschedule(&mut self);
    fn calculate_metrics(&self) -> Vec<(String, f64)>;
}

pub trait RawSingleThreadedTrigger: RawProcessor<Threading = Exclusive> {
    fn on_trigger<PC, PS>(
        &mut self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>;
}

pub trait RawMultiThreadedTrigger: RawProcessor<Threading = Concurrent> {
    fn on_trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>;
}

pub trait HasProcessorDefinition {
    fn get_definition() -> Box<dyn DynProcessorDefinition>;
}
