use crate::{
    CalculateMetrics, CffiLogger, ComponentIdentifier, Concurrent, Content,
    DynRawProcessorDefinition, LogLevel, Logger, MinifiError, OnTriggerResult, ProcessContext,
    ProcessSession, ProcessorDefinition, RawMultiThreadedTrigger, RawProcessor,
    RawProcessorDefinition, RawRegisterableProcessor, Relationship, Schedule,
};
use std::collections::HashMap;

pub struct GeneratedFlowFile<'a> {
    target_relationship: &'a Relationship,
    new_content: Option<Content<'a>>,
    attributes_to_add: HashMap<String, String>,
}

impl<'a> GeneratedFlowFile<'a> {
    pub fn new(
        target_relationship: &'a Relationship,
        new_content: Option<Content<'a>>,
        attributes_to_add: HashMap<String, String>,
    ) -> Self {
        Self {
            target_relationship,
            new_content,
            attributes_to_add,
        }
    }
}

pub trait FlowFileSource {
    fn generate<'a, Context: ProcessContext, LoggerImpl: Logger>(
        &self,
        context: &'a mut Context,
        logger: &LoggerImpl,
    ) -> Result<Option<GeneratedFlowFile<'a>>, MinifiError>;
}

#[derive(Debug)]
pub struct FlowFileSourceProcessor<Implementation>
where
    Implementation: Schedule + FlowFileSource + CalculateMetrics,
{
    logger: CffiLogger,
    scheduled_impl: Option<Implementation>,
}

impl<Implementation> RawProcessor for FlowFileSourceProcessor<Implementation>
where
    Implementation: Schedule + FlowFileSource + CalculateMetrics,
{
    type Threading = Concurrent;

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

impl<'a, Implementation> RawMultiThreadedTrigger for FlowFileSourceProcessor<Implementation>
where
    Implementation: Schedule + FlowFileSource + CalculateMetrics,
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
            if let Some(new_flow_file_data) = scheduled_impl.generate(context, &self.logger)? {
                let mut ff = session.create()?;
                if let Some(new_content) = new_flow_file_data.new_content {
                    new_content.write_to_flow_file(&mut ff, session)?;
                }
                for (k, v) in &new_flow_file_data.attributes_to_add {
                    session.set_attribute(&mut ff, k, v);
                }
                session.transfer(ff, new_flow_file_data.target_relationship.name);
                Ok(OnTriggerResult::Ok)
            } else {
                Ok(OnTriggerResult::Yield)
            }
        } else {
            Err(MinifiError::TriggerError(
                "The processor hasn't been scheduled yet".to_string(),
            ))
        }
    }
}

impl<Implementation> RawRegisterableProcessor for FlowFileSourceProcessor<Implementation>
where
    Implementation: Schedule
        + FlowFileSource
        + CalculateMetrics
        + ComponentIdentifier
        + ProcessorDefinition
        + 'static,
{
    fn get_definition() -> Box<dyn DynRawProcessorDefinition> {
        Box::new(RawProcessorDefinition::<
            FlowFileSourceProcessor<Implementation>,
        >::new(
            Implementation::CLASS_NAME,
            Implementation::DESCRIPTION,
            Implementation::INPUT_REQUIREMENT,
            Implementation::SUPPORTS_DYNAMIC_PROPERTIES,
            Implementation::SUPPORTS_DYNAMIC_RELATIONSHIPS,
            Implementation::OUTPUT_ATTRIBUTES,
            Implementation::RELATIONSHIPS,
            Implementation::PROPERTIES,
        ))
    }
}
