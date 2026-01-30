use crate::{DefaultLogger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession};

pub trait Schedulable {
    fn schedule<P: ProcessContext>(
        context: &P,
        logger: &DefaultLogger,
    ) -> Result<Self, MinifiError>
    where
        Self: Sized;

    fn unschedule(&mut self) {}
}

pub trait ConstTriggerable {
    fn trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
        logger: &DefaultLogger,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>;
}

pub trait MutTriggerable {
    fn trigger<PC, PS>(
        &mut self,
        context: &mut PC,
        session: &mut PS,
        logger: &DefaultLogger,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>;
}

pub trait MetricsProvider {
    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        vec![]
    }
}
