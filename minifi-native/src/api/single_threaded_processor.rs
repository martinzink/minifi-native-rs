use crate::api::processor_traits::CalculateMetrics;
use crate::api::raw::raw_processor::HasRawProcessorDefinition;
use crate::{
    CffiLogger, DynRawProcessorDefinition, Exclusive, LogLevel, Logger, MinifiError,
    OnTriggerResult, ProcessContext, ProcessSession, RawProcessor, RawRegisterableProcessor,
    RawSingleThreadedTrigger, Schedule,
};

pub trait MutTrigger {
    fn trigger<PC, PS, L>(
        &mut self,
        context: &mut PC,
        session: &mut PS,
        logger: &L,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
        L: Logger;
}

#[derive(Debug)]
pub struct SingleThreadedProcessor<Implementation>
where
    Implementation: Schedule + MutTrigger + HasRawProcessorDefinition + CalculateMetrics,
{
    logger: CffiLogger,
    scheduled_impl: Option<Implementation>,
}

impl<Implementation> RawProcessor for SingleThreadedProcessor<Implementation>
where
    Implementation: Schedule + MutTrigger + HasRawProcessorDefinition + CalculateMetrics,
{
    type Threading = Exclusive;

    fn new(logger: CffiLogger) -> Self {
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
        if let Some(ref scheduled_impl) = self.scheduled_impl {
            scheduled_impl.calculate_metrics()
        } else {
            self.logger
                .warn("Calculating metrics before processor is scheduled.");
            vec![]
        }
    }
}

impl<Implementation> RawSingleThreadedTrigger for SingleThreadedProcessor<Implementation>
where
    Implementation: Schedule + MutTrigger + HasRawProcessorDefinition + CalculateMetrics,
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

impl<Implementation> RawRegisterableProcessor for SingleThreadedProcessor<Implementation>
where
    Implementation: Schedule + MutTrigger + HasRawProcessorDefinition + CalculateMetrics,
{
    fn get_definition() -> Box<dyn DynRawProcessorDefinition> {
        Implementation::get_definition()
    }
}
