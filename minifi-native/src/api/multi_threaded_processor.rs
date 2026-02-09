use crate::api::RawProcessor;
use crate::api::processor_traits::CalculateMetrics;
use crate::api::raw_processor::HasProcessorDefinition;
use crate::{
    Concurrent, ConstTrigger, DefaultLogger, DynProcessorDefinition, LogLevel, Logger, MinifiError,
    OnTriggerResult, ProcessContext, ProcessSession, RawMultiThreadedTrigger,
    RawRegisterableProcessor, Schedule,
};

#[derive(Debug)]
pub struct MultiThreadedProcessor<Implementation>
where
    Implementation: Schedule + ConstTrigger + HasProcessorDefinition + CalculateMetrics,
{
    logger: DefaultLogger,
    scheduled_impl: Option<Implementation>,
}

impl<Implementation> RawProcessor for MultiThreadedProcessor<Implementation>
where
    Implementation: Schedule + ConstTrigger + HasProcessorDefinition + CalculateMetrics,
{
    type Threading = Concurrent;

    fn new(logger: DefaultLogger) -> Self {
        Self {
            logger,
            scheduled_impl: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.scheduled_impl = Some(Implementation::schedule(context, &self.logger)?);
        Ok(())
    }

    fn on_unschedule(&mut self) {
        if let Some(ref mut scheduled_impl) = self.scheduled_impl {
            scheduled_impl.unschedule()
        }
    }

    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        vec![]
    }
}

impl<Implementation> RawMultiThreadedTrigger for MultiThreadedProcessor<Implementation>
where
    Implementation: Schedule + ConstTrigger + HasProcessorDefinition + CalculateMetrics,
{
    fn on_trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
    {
        if let Some(ref scheduled_impl) = self.scheduled_impl {
            scheduled_impl.trigger(context, session, &self.logger)
        } else {
            Err(MinifiError::TriggerError(
                "The processor hasnt been scheduled yet".to_string(),
            ))
        }
    }
}

impl<Implementation> RawRegisterableProcessor for MultiThreadedProcessor<Implementation>
where
    Implementation: Schedule + ConstTrigger + HasProcessorDefinition + CalculateMetrics,
{
    fn get_definition() -> Box<dyn DynProcessorDefinition> {
        Implementation::get_definition()
    }
}
