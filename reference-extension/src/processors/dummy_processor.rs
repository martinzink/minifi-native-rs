mod properties;

use minifi_native::{Concurrent, ConcurrentOnTrigger, DefaultLogger, LogLevel, Logger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, Processor};
use crate::controller_services::dummy_controller_service::DummyControllerService;
use crate::processors::dummy_processor::properties::CONTROLLER_SERVICE;

#[derive(Debug)]
pub(crate) struct DummyProcessor {
    logger: DefaultLogger,
    dummy_controller_service_name: Option<String>
}

impl Processor for DummyProcessor {
    type Threading = Concurrent;

    fn new(logger: DefaultLogger) -> Self {
        Self { logger,dummy_controller_service_name: None }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message)
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.dummy_controller_service_name = context.get_property(&CONTROLLER_SERVICE, None)?;
        Ok(())
    }
}

impl ConcurrentOnTrigger for DummyProcessor {
    fn on_trigger<PC, PS>(&self, context: &mut PC, _session: &mut PS) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile=PC::FlowFile>
    {
        println!("DummyProcessor::on_trigger {:?}", self);
        let cs = context.get_controller_service::<DummyControllerService>(&CONTROLLER_SERVICE)?;
        println!("{:?}", cs);
        Ok(OnTriggerResult::Ok)
    }
}

#[cfg(not(test))]
pub(crate) mod processor_definition;
mod relationships;