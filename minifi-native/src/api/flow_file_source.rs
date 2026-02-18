use crate::{
    CalculateMetrics, ComponentIdentifier, Concurrent, Content, DynRawProcessorDefinition, Logger,
    MinifiError, OnTriggerResult, ProcessContext, ProcessSession, Processor, ProcessorDefinition,
    RawMultiThreadedTrigger, RawProcessorDefinition, RawRegisterableProcessor, Relationship,
    Schedule,
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

pub struct FlowFileSourceProcessorType {}

impl<'a, Implementation> RawMultiThreadedTrigger
    for Processor<Implementation, FlowFileSourceProcessorType, Concurrent>
where
    Implementation: Schedule + CalculateMetrics + FlowFileSource,
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

impl<Implementation> RawRegisterableProcessor
    for Processor<Implementation, FlowFileSourceProcessorType, Concurrent>
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
            Processor<Implementation, FlowFileSourceProcessorType, Concurrent>,
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
