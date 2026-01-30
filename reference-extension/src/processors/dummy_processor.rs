mod properties;
mod relationships;

use crate::controller_services::dummy_controller_service::DummyControllerService;
use crate::processors::dummy_processor::properties::CONTROLLER_SERVICE;
use minifi_native::{DefaultLogger, Logger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, Schedulable, ConstTriggerable, MetricsProvider};

#[derive(Debug)]
pub(crate) struct DummyProcessor {
}

impl Schedulable for DummyProcessor {
    fn schedule<P: ProcessContext>(_context: &P, _logger: &DefaultLogger) -> Result<Self, MinifiError>
    where
        Self: Sized
    {
        Ok(Self{})
    }
}

impl ConstTriggerable for DummyProcessor {
    fn trigger<PC, PS>(&self, context: &mut PC, _session: &mut PS, logger: &DefaultLogger) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile=PC::FlowFile>
    {
        match context.get_controller_service::<DummyControllerService>(&CONTROLLER_SERVICE)? {
            None => {
                logger
                    .info("Couldnt not get information about DummyControllerService");
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

impl MetricsProvider for DummyProcessor {}

#[cfg(not(test))]
pub(crate) mod processor_definition;
