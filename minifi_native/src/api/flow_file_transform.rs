use crate::api::context_session_flowfile_bundle::ContextSessionFlowFileBundle;
use crate::api::flow_file_content::Content;
use crate::api::processor::Processor;
use crate::api::property::{GetControllerService, GetProperty};
use crate::api::raw::raw_processor::RawMultiThreadedTrigger;
use crate::api::{InputStream, ProcessorDefinition, RawProcessor};
use crate::c_ffi::{DynRawProcessorDefinition, RawProcessorDefinition, RawRegisterableProcessor};
use crate::{
    CalculateMetrics, ComponentIdentifier, Concurrent, GetAttribute, LogLevel, Logger, MinifiError,
    OnTriggerResult, ProcessContext, ProcessSession, Relationship, Schedule,
};
use std::collections::HashMap;

pub struct TransformedFlowFile<'a> {
    target_relationship_name: &'static str,
    new_content: Option<Content<'a>>,
    attributes_to_add: HashMap<String, String>,
}

impl<'a> TransformedFlowFile<'a> {
    pub fn route_without_changes(target_relationship: &Relationship) -> Self {
        Self {
            target_relationship_name: target_relationship.name,
            new_content: None,
            attributes_to_add: HashMap::new(),
        }
    }
    pub fn new(
        target_relationship: &Relationship,
        new_content: Option<Vec<u8>>,
        attributes_to_add: HashMap<String, String>,
    ) -> Self {
        Self {
            target_relationship_name: target_relationship.name,
            new_content: Some(Content::Buffer(new_content.unwrap_or_default())),
            attributes_to_add,
        }
    }

    pub fn new_content(&'_ self) -> Option<&'_ Content<'_>> {
        self.new_content.as_ref()
    }

    pub fn target_relationship(&self) -> &'static str {
        self.target_relationship_name
    }

    pub fn attributes_to_add(&self) -> &HashMap<String, String> {
        &self.attributes_to_add
    }
}

pub trait FlowFileTransform {
    fn transform<
        'ctx,
        'stream,
        Context: GetProperty + GetControllerService + GetAttribute,
        LoggerImpl: Logger,
    >(
        &self,
        context: &'ctx Context,
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
                let simple_context =
                    ContextSessionFlowFileBundle::new(context, session, Some(&flow_file));
                let (attrs_to_add, relationship) =
                    session.read_stream(&flow_file, |input_stream| {
                        let transformed = scheduled_impl.transform(
                            &simple_context,
                            input_stream,
                            &self.logger,
                        )?;

                        match transformed.new_content {
                            None => {}
                            Some(Content::Buffer(buffer)) => {
                                session.write(&flow_file, &buffer)?;
                            }
                            Some(Content::Stream(stream)) => {
                                session.write_lazy(&flow_file, stream)?;
                            }
                        };
                        Ok((
                            transformed.attributes_to_add,
                            transformed.target_relationship_name,
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
            Err(MinifiError::trigger_err(
                "The processor hasn't been scheduled yet",
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
