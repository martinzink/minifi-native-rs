use crate::{
    Logger, MinifiError, OnTriggerResult, OutputAttribute, ProcessContext, ProcessSession,
    ProcessorInputRequirement, Property, Relationship,
};

pub trait Schedule {
    fn schedule<P: ProcessContext, L: Logger>(context: &P, logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized;

    fn unschedule(&mut self) {}
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

pub trait CalculateMetrics {
    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        vec![]
    }
}

pub trait ProcessorDefinition {
    const DESCRIPTION: &'static str;
    const INPUT_REQUIREMENT: ProcessorInputRequirement;
    const SUPPORTS_DYNAMIC_PROPERTIES: bool;
    const SUPPORTS_DYNAMIC_RELATIONSHIPS: bool;
    const OUTPUT_ATTRIBUTES: &'static [OutputAttribute];
    const RELATIONSHIPS: &'static [Relationship];
    const PROPERTIES: &'static [Property];
}
