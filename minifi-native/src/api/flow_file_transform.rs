// WORK IN PROGRESS removed from build
use crate::{Concurrent, ConstTriggerable, DefaultLogger, DynProcessorDefinition, HasProcessorDefinition, LogLevel, Logger, MetricsProvider, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, RawMultiThreadedTrigger, RawProcessor, RawRegisterableProcessor, Relationship, Schedulable};

struct TransformResult<'a, FlowFileType> {
    flow_file: FlowFileType,
    relationship: &'a Relationship,
    content: Option<Vec<u8>>,
    attributes: Vec<(String, String)>,
}

pub trait ConstFlowFileTransform {
    fn transform<Context: ProcessContext, LoggerImpl: Logger>(
        &self,
        context: &mut Context,
        flow_file: Context::FlowFile,
        logger: &LoggerImpl,
    ) -> TransformResult<'_, Context::FlowFile>;
}

pub trait MutFlowFileTransform {
    fn transform<Context: ProcessContext, LoggerImpl: Logger>(
        &mut self,
        context: &mut Context,
        flow_file: Context::FlowFile,
        logger: &LoggerImpl,
    ) -> TransformResult<'_, Context::FlowFile>;
}

#[derive(Debug)]
pub struct MultiThreadedFlowFileTransformer<Implementation>
where
    Implementation: Schedulable + ConstFlowFileTransform + HasProcessorDefinition + MetricsProvider,
{
    logger: DefaultLogger,
    scheduled_impl: Option<Implementation>,
}

impl<Implementation> RawProcessor for MultiThreadedFlowFileTransformer<Implementation>
where
    Implementation: Schedulable + ConstFlowFileTransform + HasProcessorDefinition + MetricsProvider,
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
}

impl<Implementation> RawMultiThreadedTrigger for MultiThreadedFlowFileTransformer<Implementation>
where
    Implementation: Schedulable + ConstFlowFileTransform + HasProcessorDefinition + MetricsProvider,
{
    fn on_trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile= PC::FlowFile>,
    {
        if let Some(ref scheduled_impl) = self.scheduled_impl {
            if let Some(flow_file) = session.get() {
                let transform_result = scheduled_impl.transform(context, flow_file, &self.logger);

            } else {
                self.log(LogLevel::Trace, "No flowfile to transform");
                Ok(OnTriggerResult::Yield)
            }
        } else {
            Err(MinifiError::TriggerError(
                "The processor hasnt been scheduled yet".to_string(),
            ))
        }
    }
}

impl<Implementation> RawRegisterableProcessor for MultiThreadedFlowFileTransformer<Implementation>
where
    Implementation: Schedulable + ConstFlowFileTransform + HasProcessorDefinition + MetricsProvider,
{
    fn get_definition() -> Box<dyn DynProcessorDefinition> {
        Implementation::get_definition()
    }
}
