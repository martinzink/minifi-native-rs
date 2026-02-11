mod properties;

use crate::controller_services::dummy_controller_service::properties::DATA;
use minifi_native::{
    ControllerService, ControllerServiceContext, DefaultLogger, IdentifyComponent, LogLevel,
    Logger, MinifiError,
};

#[derive(Debug, IdentifyComponent)]
pub(crate) struct DummyControllerService {
    logger: DefaultLogger,
    data: Option<String>,
}

impl ControllerService for DummyControllerService {
    fn new(logger: DefaultLogger) -> Self {
        DummyControllerService { logger, data: None }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.data = context.get_property(&DATA)?;
        self.logger
            .info(format!("DummyControllerService::enable {:?}", self).as_str());

        Ok(())
    }

    fn disable(&mut self) {
        self.logger
            .info(format!("DummyControllerService::disable {:?}", self).as_str());
    }
}

impl DummyControllerService {
    pub fn get_data(&self) -> Option<&str> {
        self.data.as_deref()
    }
}

#[cfg(not(test))]
mod controller_service_definition;

#[cfg(test)]
mod tests;
