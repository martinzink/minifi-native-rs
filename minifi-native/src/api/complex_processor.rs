use crate::api::processor_traits::CalculateMetrics;
use crate::{
    ComponentIdentifier, Concurrent, DynRawProcessorDefinition, Exclusive, Logger, MinifiError,
    OnTriggerResult, ProcessContext, ProcessSession, Processor, ProcessorDefinition,
    RawMultiThreadedTrigger, RawProcessorDefinition, RawRegisterableProcessor,
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

pub trait ConstTrigger {
    fn trigger<PC, PS, L>(
        &self,
        context: &mut PC,
        session: &mut PS,
        logger: &L,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
        L: Logger;
}

pub struct ComplexProcessorType {}

impl<Implementation> RawSingleThreadedTrigger
    for Processor<Implementation, ComplexProcessorType, Exclusive>
where
    Implementation:
        Schedule + MutTrigger + ComponentIdentifier + ProcessorDefinition + CalculateMetrics,
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

impl<Implementation> RawMultiThreadedTrigger
    for Processor<Implementation, ComplexProcessorType, Concurrent>
where
    Implementation:
        Schedule + ConstTrigger + ComponentIdentifier + ProcessorDefinition + CalculateMetrics,
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

impl<Implementation> RawRegisterableProcessor
    for Processor<Implementation, ComplexProcessorType, Exclusive>
where
    Implementation: Schedule
        + MutTrigger
        + ComponentIdentifier
        + ProcessorDefinition
        + CalculateMetrics
        + 'static,
{
    fn get_definition() -> Box<dyn DynRawProcessorDefinition> {
        Box::new(RawProcessorDefinition::<
            Processor<Implementation, ComplexProcessorType, Exclusive>,
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

impl<Implementation> RawRegisterableProcessor
    for Processor<Implementation, ComplexProcessorType, Concurrent>
where
    Implementation: Schedule
        + ConstTrigger
        + ComponentIdentifier
        + ProcessorDefinition
        + CalculateMetrics
        + 'static,
{
    fn get_definition() -> Box<dyn DynRawProcessorDefinition> {
        Box::new(RawProcessorDefinition::<
            Processor<Implementation, ComplexProcessorType, Concurrent>,
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
