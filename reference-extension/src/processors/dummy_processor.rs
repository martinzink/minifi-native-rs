mod properties;
use crate::controller_services::dummy_controller_service::DummyControllerService;
use crate::processors::dummy_processor::properties::CONTROLLER_SERVICE;
use minifi_native::{
    ConstTrigger, Logger, CalculateMetrics, MinifiError, OnTriggerResult, ProcessContext,
    ProcessSession, Schedule,
};

#[derive(Debug)]
pub(crate) struct DummyProcessor {}

impl Schedule for DummyProcessor {
    fn schedule<P: ProcessContext, L: Logger>(
        _context: &P,
        _logger: &L,
    ) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}

impl ConstTrigger for DummyProcessor {
    fn trigger<PC, PS, L>(
        &self,
        context: &mut PC,
        _session: &mut PS,
        logger: &L,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
        L: Logger,
    {
        match context.get_controller_service::<DummyControllerService>(&CONTROLLER_SERVICE)? {
            None => {
                logger.info("Couldnt not get information about DummyControllerService");
            }
            Some(dummy_controller) => {
                logger.info(
                    format!(
                        "The data in the DummyControllerService is {:?}",
                        dummy_controller.get_data()
                    )
                    .as_str(),
                );
            }
        }
        Ok(OnTriggerResult::Ok)
    }
}

impl CalculateMetrics for DummyProcessor {}

#[cfg(not(test))]
pub(crate) mod processor_definition;

#[cfg(test)]
mod tests;
