use crate::{DefaultLogger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession};

pub trait ConstTriggerable {
    fn trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
        logger: &DefaultLogger
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>;
}