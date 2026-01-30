use crate::{RawMultiThreadedTrigger, DefaultLogger, DynProcessorDefinition, LogLevel, Logger, MinifiError, MutTriggerable, Schedulable, OnTriggerResult, ProcessContext, ProcessSession, RawProcessor, RegisterableProcessor, Exclusive, RawSingleThreadedTrigger};
use crate::api::processor::HasProcessorDefinition;

#[derive(Debug)]
pub struct SingleThreadedProcessor<Implementation>
where
    Implementation: Schedulable + MutTriggerable + HasProcessorDefinition,
{
    logger: DefaultLogger,
    scheduled_impl: Option<Implementation>,
}

impl<Implementation> RawProcessor for SingleThreadedProcessor<Implementation>
where
    Implementation: Schedulable + MutTriggerable + HasProcessorDefinition,
{
    type Threading = Exclusive;

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
}

impl<Implementation> RawSingleThreadedTrigger for SingleThreadedProcessor<Implementation>
where
    Implementation: Schedulable + MutTriggerable + HasProcessorDefinition,
{
    fn on_trigger<PC, PS>(
        &mut self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
    {
        if let Some(ref mut scheduled_impl) = self.scheduled_impl {
            scheduled_impl.trigger(context, session, &self.logger)
        } else {
            Err(MinifiError::TriggerError(
                "The processor hasnt been scheduled yet".to_string(),
            ))
        }
    }
}

impl<Implementation> RegisterableProcessor for SingleThreadedProcessor<Implementation>
where
    Implementation: Schedulable + MutTriggerable + HasProcessorDefinition,
{
    fn get_definition() -> Box<dyn DynProcessorDefinition> {
        Implementation::get_definition()
    }
}
