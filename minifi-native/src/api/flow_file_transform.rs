use crate::{CalculateMetrics, Concurrent, DefaultLogger, DynProcessorDefinition, FlowFile, HasProcessorDefinition, LogLevel, Logger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, RawMultiThreadedTrigger, RawProcessor, RawRegisterableProcessor, Relationship, Schedule};

pub struct TransformedFlowFile<'a, FlowFileType> {
    flow_file: FlowFileType,
    target_relationship: &'a Relationship,
    new_content: Option<Vec<u8>>,
    attributes_to_add: Vec<(String, String)>,
}

pub enum TransformError<FlowFileType> {
    RouteTo((FlowFileType, &'static str)),
    MinifiError(MinifiError),
}

pub trait ConstFlowFileTransform {
    fn transform<Context: ProcessContext, LoggerImpl: Logger>(
        &self,
        context: &mut Context,
        flow_file: Context::FlowFile,
        logger: &LoggerImpl,
    ) -> Result<TransformedFlowFile<'_, Context::FlowFile>, TransformError<Context::FlowFile>>;
}

#[derive(Debug)]
pub struct MultiThreadedFlowFileTransformer<Implementation>
where
    Implementation: Schedule + ConstFlowFileTransform + HasProcessorDefinition + CalculateMetrics,
{
    logger: DefaultLogger,
    scheduled_impl: Option<Implementation>,
}

impl<Implementation> RawProcessor for MultiThreadedFlowFileTransformer<Implementation>
where
    Implementation: Schedule + ConstFlowFileTransform + HasProcessorDefinition + CalculateMetrics,
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
    Implementation: Schedule + ConstFlowFileTransform + HasProcessorDefinition + CalculateMetrics,
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
                match scheduled_impl.transform(context, flow_file, &self.logger) {
                    Ok(mut transform_result) => {
                        for (k, v) in &transform_result.attributes_to_add {
                            session.set_attribute(&mut transform_result.flow_file, k, v);
                        }
                        if let Some(new_content) = &transform_result.new_content {
                            session.write(&mut transform_result.flow_file, new_content);
                        }
                        session.transfer(transform_result.flow_file, transform_result.target_relationship.name);
                        Ok(OnTriggerResult::Ok)
                    },
                    Err(TransformError::<PC::FlowFile>::RouteTo((ff, target))) => {
                        session.transfer(ff, target);
                        Ok(OnTriggerResult::Ok)
                    },
                    Err(TransformError::<PC::FlowFile>::MinifiError(e)) => Err(e)
                }
            } else {
                self.log(LogLevel::Trace, "No flowfile to transform");
                Ok(OnTriggerResult::Yield)
            }
        } else {
            Err(MinifiError::TriggerError(
                "The processor hasn't been scheduled yet".to_string(),
            ))
        }
    }
}

impl<Implementation> RawRegisterableProcessor for MultiThreadedFlowFileTransformer<Implementation>
where
    Implementation: Schedule + ConstFlowFileTransform + HasProcessorDefinition + CalculateMetrics,
{
    fn get_definition() -> Box<dyn DynProcessorDefinition> {
        Implementation::get_definition()
    }
}
