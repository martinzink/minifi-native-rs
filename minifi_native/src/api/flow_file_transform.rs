use crate::api::flow_file_content::Content;
use crate::api::processor::Processor;
use crate::api::raw::raw_processor::RawMultiThreadedTrigger;
use crate::api::{InputStream, ProcessorDefinition, RawProcessor};
use crate::c_ffi::{DynRawProcessorDefinition, RawProcessorDefinition, RawRegisterableProcessor};
use crate::{
    CalculateMetrics, ComponentIdentifier, Concurrent, LogLevel, Logger, MinifiError,
    OnTriggerResult, ProcessContext, ProcessSession, Relationship, Schedule,
};
use std::collections::HashMap;

pub struct TransformedFlowFile<'a> {
    target_relationship: &'a Relationship,
    new_content: Option<Content<'a>>,
    attributes_to_add: HashMap<String, String>,
}

impl<'a> TransformedFlowFile<'a> {
    pub fn route_without_changes(target_relationship: &'a Relationship) -> Self {
        Self {
            target_relationship,
            new_content: None,
            attributes_to_add: HashMap::new(),
        }
    }
    pub fn new(
        target_relationship: &'a Relationship,
        new_content: Option<Vec<u8>>,
        attributes_to_add: HashMap<String, String>,
    ) -> Self {
        Self {
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
    fn transform<'ctx, 'stream, Context: ProcessContext, LoggerImpl: Logger>(
        &self,
        context: &'ctx mut Context,
        _flow_file: &Context::FlowFile,
        input_stream: &'stream mut dyn InputStream,
        logger: &LoggerImpl,
    ) -> Result<TransformedFlowFile<'stream>, MinifiError>
    where
        'ctx: 'stream;
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
            if let Some(mut flow_file) = session.get() {
                let (attrs_to_add, relationship) =
                    session.read_stream(&flow_file, |input_stream, ff| {
                        let transformed = scheduled_impl.transform(
                            context,
                            &flow_file,
                            input_stream,
                            &self.logger,
                        )?;

                        match transformed.new_content {
                            None => {}
                            Some(Content::Buffer(buffer)) => {
                                session.write(ff, &buffer)?;
                            }
                            Some(Content::Stream(stream)) => {
                                session.write_stream(ff, stream)?;
                            }
                        };
                        Ok((
                            transformed.attributes_to_add,
                            transformed.target_relationship.name,
                        ))
                    })?;

                for (k, v) in attrs_to_add {
                    session.set_attribute(&mut flow_file, &k, &v)?;
                }

                session.transfer(flow_file, relationship)?;

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
