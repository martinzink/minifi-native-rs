use minifi_native::{MinifiError, LogLevel, Logger, ProcessContext, ProcessSession, Processor, Property};

mod relationships;
mod properties;


#[derive(Debug)]
struct GetFile<L: Logger> {
    logger: L,
}



impl<L: Logger> Processor<L> for GetFile<L> {
    fn new(logger: L) -> Self {
        Self {
            logger,
        }
    }

    fn on_trigger<P, S>(&mut self, _context: &P, session: &mut S) -> Result<(), MinifiError>
    where
        P: ProcessContext,
        S: ProcessSession,
    {
        Ok(())
    }


    fn on_schedule<P>(&mut self, context: &P) -> Result<(), MinifiError>
    where
        P: ProcessContext,
    {
        Ok(())
    }

    fn log(&mut self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }
}

#[cfg(not(test))]
mod register_ctor;

#[cfg(test)]
mod tests;
