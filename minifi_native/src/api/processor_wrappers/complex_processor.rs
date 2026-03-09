use crate::api::processor::{AdvancedProcessorFeatures, CalculateMetrics};
use crate::api::raw_processor::{MultiThreadedTrigger, SingleThreadedTrigger};
use crate::{
    ComponentIdentifier, Concurrent, Exclusive, Logger, MinifiError, OnTriggerResult,
    ProcessContext, ProcessSession, Processor, ProcessorDefinition, Schedule,
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

pub trait Trigger {
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

impl<Implementation, L> SingleThreadedTrigger
    for Processor<Implementation, ComplexProcessorType, Exclusive, L>
where
    Implementation: Schedule
        + MutTrigger
        + ComponentIdentifier
        + ProcessorDefinition
        + CalculateMetrics
        + AdvancedProcessorFeatures,
    L: Logger,
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
            Err(MinifiError::trigger_err(
                "The processor hasnt been scheduled yet",
            ))
        }
    }
}

impl<Implementation, L> MultiThreadedTrigger
    for Processor<Implementation, ComplexProcessorType, Concurrent, L>
where
    Implementation: Schedule
        + Trigger
        + ComponentIdentifier
        + ProcessorDefinition
        + CalculateMetrics
        + AdvancedProcessorFeatures,
    L: Logger,
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
            Err(MinifiError::trigger_err(
                "The processor hasnt been scheduled yet",
            ))
        }
    }
}
