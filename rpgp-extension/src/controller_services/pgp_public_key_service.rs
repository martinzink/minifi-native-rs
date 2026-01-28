use minifi_native::{ControllerService, ControllerServiceContext, DefaultLogger, LogLevel, Logger, MinifiError};
use pgp::composed::Deserializable;

#[derive(Debug)]
pub(crate) struct PgpPublicKeyService {
    logger: DefaultLogger,
    public_key: Option<u64>
}

impl ControllerService for PgpPublicKeyService {
    fn new(logger: DefaultLogger) -> Self {
        PgpPublicKeyService{logger, public_key: None}
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        Ok(())
    }

    fn disable(&mut self) {
        todo!()
    }

    fn class_name() -> &'static str {
        todo!()
    }

    fn group_name() -> &'static str {
        todo!()
    }

    fn version() -> &'static str {
        todo!()
    }
}