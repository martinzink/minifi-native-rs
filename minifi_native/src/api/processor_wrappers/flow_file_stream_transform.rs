use crate::api::RawProcessor;
use crate::api::process_session::IoState;
use crate::api::processor::AdvancedProcessorFeatures;
use crate::api::processor_wrappers::utils::context_session_flowfile_bundle::ContextSessionFlowFileBundle;
use crate::api::raw::raw_processor::RawMultiThreadedTrigger;
use crate::c_ffi::{DynRawProcessorDefinition, RawProcessorDefinition, RawRegisterableProcessor};
use crate::{
    CalculateMetrics, ComponentIdentifier, Concurrent, GetAttribute, GetControllerService,
    GetProperty, InputStream, LogLevel, Logger, MinifiError, OnTriggerResult, OutputStream,
    ProcessContext, ProcessSession, Processor, ProcessorDefinition, Relationship, Schedule,
};
use std::collections::HashMap;

pub struct TransformStreamResult {
    target_relationship_name: &'static str,
    attributes_to_add: HashMap<String, String>,
    write_status: IoState,
}

impl TransformStreamResult {
    pub fn new(
        target_relationship: &Relationship,
        attributes_to_add: HashMap<String, String>,
    ) -> Self {
        Self {
            target_relationship_name: target_relationship.name,
            attributes_to_add,
            write_status: IoState::Ok,
        }
    }

    pub fn route_without_changes(target_relationship: &Relationship) -> Self {
        Self {
            target_relationship_name: target_relationship.name,
            attributes_to_add: HashMap::new(),
            write_status: IoState::Cancel,
        }
    }

    pub fn target_relationship_name(&self) -> &'static str {
        self.target_relationship_name
    }

    pub fn get_attribute(&self, name: &str) -> Option<String> {
        self.attributes_to_add.get(name).cloned()
    }

    pub fn write_status(&self) -> IoState {
        self.write_status
    }
}

pub trait FlowFileStreamTransform {
    fn transform<Ctx: GetProperty + GetControllerService + GetAttribute, LoggerImpl: Logger>(
        &self,
        context: &Ctx,
        input_stream: &mut dyn InputStream,
        output_stream: &mut dyn OutputStream,
        logger: &LoggerImpl,
    ) -> Result<TransformStreamResult, MinifiError>;
}

pub struct FlowFileStreamTransformProcessorType {}

impl<'a, Implementation> RawMultiThreadedTrigger
    for Processor<Implementation, FlowFileStreamTransformProcessorType, Concurrent>
where
    Implementation:
        Schedule + CalculateMetrics + FlowFileStreamTransform + AdvancedProcessorFeatures,
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
                let (relationship, attrs) = session.read_stream(&flow_file, |input_stream| {
                    session.write_stream(&flow_file, |output_stream| {
                        let transformed = scheduled_impl.transform(
                            &simple_context,
                            input_stream,
                            output_stream,
                            &self.logger,
                        )?;

                        Ok((
                            (
                                transformed.target_relationship_name,
                                transformed.attributes_to_add,
                            ),
                            transformed.write_status,
                        ))
                    })
                })?;
                for (k, v) in attrs {
                    session.set_attribute(&mut flow_file, &k, &v)?;
                }

                session.transfer(flow_file, relationship)?;

                Ok(OnTriggerResult::Ok)
            } else {
                self.log(LogLevel::Trace, format_args!("No flowfile to transform"));
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
    for Processor<Implementation, FlowFileStreamTransformProcessorType, Concurrent>
where
    Implementation: Schedule
        + FlowFileStreamTransform
        + CalculateMetrics
        + ComponentIdentifier
        + ProcessorDefinition
        + AdvancedProcessorFeatures
        + 'static,
{
    fn get_definition() -> Box<dyn DynRawProcessorDefinition> {
        Box::new(RawProcessorDefinition::<
            Processor<Implementation, FlowFileStreamTransformProcessorType, Concurrent>,
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
