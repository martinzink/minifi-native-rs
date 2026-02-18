use crate::api::ProcessorDefinition;
use crate::api::flow_file_content::Content;
use crate::api::processor_traits::Processor;
use crate::{
    CalculateMetrics, ComponentIdentifier, Concurrent, DynRawProcessorDefinition, LogLevel, Logger,
    MinifiError, OnTriggerResult, ProcessContext, ProcessSession, RawMultiThreadedTrigger,
    RawProcessor, RawProcessorDefinition, RawRegisterableProcessor, Relationship, Schedule,
};
use std::collections::HashMap;

pub struct TransformedFlowFile<'a, FlowFileType> {
    flow_file: FlowFileType,
    target_relationship: &'a Relationship,
    new_content: Option<Content<'a>>,
    attributes_to_add: HashMap<String, String>,
}

impl<'a, FlowFileType> TransformedFlowFile<'a, FlowFileType> {
    pub fn route_without_changes(
        flow_file: FlowFileType,
        target_relationship: &'a Relationship,
    ) -> Self {
        Self {
            flow_file,
            target_relationship,
            new_content: None,
            attributes_to_add: HashMap::new(),
        }
    }
    pub fn new(
        flow_file: FlowFileType,
        target_relationship: &'a Relationship,
        new_content: Option<Vec<u8>>,
        attributes_to_add: HashMap<String, String>,
    ) -> Self {
        Self {
            flow_file,
            target_relationship,
            new_content: Some(Content::Buffer(new_content.unwrap_or_default())),
            attributes_to_add,
        }
    }

    pub fn new_content(&'_ self) -> Option<&'_ Content<'_>> {
        self.new_content.as_ref()
    }

    pub fn target_relationship(&self) -> &Relationship {
        self.target_relationship
    }

    pub fn attributes_to_add(&self) -> &HashMap<String, String> {
        &self.attributes_to_add
    }
}

pub trait FlowFileTransform {
    fn transform<
        'a,
        Context: ProcessContext,
        GetContent: FnMut(&Context::FlowFile) -> Option<Vec<u8>>,
        LoggerImpl: Logger,
    >(
        &self,
        context: &'a mut Context,
        flow_file: Context::FlowFile,
        flow_file_content: GetContent,
        logger: &LoggerImpl,
    ) -> Result<TransformedFlowFile<'a, Context::FlowFile>, MinifiError>;
}

pub struct FlowFileTransformProcessorType {}

impl<'a, Implementation> RawMultiThreadedTrigger
    for Processor<Implementation, FlowFileTransformProcessorType, Concurrent>
where
    Implementation: Schedule + CalculateMetrics + FlowFileTransform,
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
            if let Some(flow_file) = session.get() {
                let mut transformed_ff = scheduled_impl.transform(
                    context,
                    flow_file,
                    |ff| session.read(&ff),
                    &self.logger,
                )?;
                for (k, v) in &transformed_ff.attributes_to_add {
                    session.set_attribute(&mut transformed_ff.flow_file, k, v);
                }
                if let Some(new_content) = transformed_ff.new_content {
                    new_content.write_to_flow_file(&mut transformed_ff.flow_file, session)?;
                }

                session.transfer(
                    transformed_ff.flow_file,
                    transformed_ff.target_relationship.name,
                );
                Ok(OnTriggerResult::Ok)
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

impl<Implementation> RawRegisterableProcessor
    for Processor<Implementation, FlowFileTransformProcessorType, Concurrent>
where
    Implementation: Schedule
        + FlowFileTransform
        + CalculateMetrics
        + ComponentIdentifier
        + ProcessorDefinition
        + 'static,
{
    fn get_definition() -> Box<dyn DynRawProcessorDefinition> {
        Box::new(RawProcessorDefinition::<
            Processor<Implementation, FlowFileTransformProcessorType, Concurrent>,
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
