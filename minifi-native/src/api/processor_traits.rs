use crate::{Logger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession};

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
