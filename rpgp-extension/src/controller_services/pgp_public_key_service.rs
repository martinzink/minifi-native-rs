use minifi_native::{ControllerService, ControllerServiceContext, LogLevel, Logger, MinifiError};
use pgp::composed::Deserializable;

#[derive(Debug)]
pub(crate) struct PgpPublicKeyService<L: Logger> {
    logger: L,
    public_key: Option<u64>
}

impl<L: Logger> ControllerService<L> for PgpPublicKeyService<L> {
    fn new(logger: L) -> Self {
        PgpPublicKeyService{logger, public_key: None}
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        Ok(())
    }
}