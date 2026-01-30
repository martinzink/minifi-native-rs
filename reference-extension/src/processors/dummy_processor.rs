mod properties;
mod relationships;

use crate::controller_services::dummy_controller_service::DummyControllerService;
use crate::processors::dummy_processor::properties::CONTROLLER_SERVICE;
use minifi_native::{
    Concurrent, RawMultiThreadedTrigger, DefaultLogger, LogLevel, Logger, MinifiError, OnTriggerResult,
    ProcessContext, ProcessSession, RawProcessor,
};

#[derive(Debug)]
pub(crate) struct DummyProcessor {
    logger: DefaultLogger,
    dummy_controller_service_name: Option<String>,
}

impl RawProcessor for DummyProcessor {
    type Threading = Concurrent;

    fn new(logger: DefaultLogger) -> Self {
        Self {
            logger,
            dummy_controller_service_name: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message)
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.dummy_controller_service_name = context.get_property(&CONTROLLER_SERVICE, None)?;
        Ok(())
    }
}

impl RawMultiThreadedTrigger for DummyProcessor {
    fn on_trigger<PC, PS>(
        &self,
        context: &mut PC,
        _session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
    {
        match context.get_controller_service::<DummyControllerService>(&CONTROLLER_SERVICE)? {
            None => {
                self.logger
                    .info("Couldnt not get information about DummyControllerService");
            }
            Some(dummy_controller) => {
                self.logger.info(
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

#[cfg(not(test))]
pub(crate) mod processor_definition;
#[cfg(test)]
mod test;
