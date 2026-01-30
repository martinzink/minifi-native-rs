use crate::{Concurrent, ConcurrentOnTrigger, DefaultLogger, DynProcessorDefinition, LogLevel, Logger, MinifiError, NextConcurrentOnTrigger, NextGenProcessor, OnTriggerResult, ProcessContext, ProcessSession, RawProcessor, RegisterableProcessor};
use crate::api::processor::Registerable;

#[derive(Debug)]
pub struct MultiThreadedProcessor<Implementation>
where
    Implementation: NextGenProcessor<Threading = Concurrent>,
{
    logger: DefaultLogger,
    scheduled_impl: Option<Implementation>,
}

impl<Implementation> RawProcessor for MultiThreadedProcessor<Implementation>
where
    Implementation: NextGenProcessor<Threading = Concurrent>,
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
}

impl<Implementation> ConcurrentOnTrigger for MultiThreadedProcessor<Implementation>
where
    Implementation: NextGenProcessor<Threading = Concurrent> + NextConcurrentOnTrigger,
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

impl<Implementation> RegisterableProcessor for MultiThreadedProcessor<Implementation>
where Implementation: Registerable + NextGenProcessor<Threading = Concurrent> {
    fn get_definition() -> Box<dyn DynProcessorDefinition> {
        Implementation::get_definition()
    }
}
