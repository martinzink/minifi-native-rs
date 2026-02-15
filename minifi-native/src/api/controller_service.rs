use crate::{CffiControllerServiceDefinition, ComponentIdentifier, ControllerServiceContext, ControllerServiceDefinition, DefaultLogger, DynRawControllerServiceDefinition, LogLevel, Logger, MinifiError, RawControllerService, RegisterableControllerService};

pub trait EnableControllerService {
    fn enable<P: ControllerServiceContext, L: Logger>(
        context: &P,
        logger: &L,
    ) -> Result<Self, MinifiError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct ControllerService<Implementation>
where
    Implementation: EnableControllerService + ComponentIdentifier,
{
    logger: DefaultLogger,
    enabled_impl: Option<Implementation>,
}

impl<Implementation> ControllerService<Implementation>
where
    Implementation: EnableControllerService + ComponentIdentifier,
{
    pub fn get_implementation(&self) -> Option<&Implementation> {
        self.enabled_impl.as_ref()
    }
}

impl<Implementation> RawControllerService for ControllerService<Implementation>
where
    Implementation: EnableControllerService + ComponentIdentifier,
{
    fn new(logger: DefaultLogger) -> Self {
        Self {
            logger,
            enabled_impl: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.enabled_impl = Some(Implementation::enable(context, &self.logger)?);
        Ok(())
    }
}

impl<Implementation> ComponentIdentifier for ControllerService<Implementation>
where
    Implementation: EnableControllerService + ComponentIdentifier,
{
    const CLASS_NAME: &'static str = Implementation::CLASS_NAME;
    const GROUP_NAME: &'static str = Implementation::GROUP_NAME;
    const VERSION: &'static str = Implementation::VERSION;
}

impl<Implementation> RegisterableControllerService for ControllerService<Implementation>
where
    Implementation: EnableControllerService + ComponentIdentifier + ControllerServiceDefinition + 'static,
{
    fn get_definition() -> Box<dyn DynRawControllerServiceDefinition> {
        Box::new(CffiControllerServiceDefinition::<ControllerService<Implementation>>::new(
            Implementation::DESCRIPTION,
            Implementation::PROPERTIES,
        ))
    }
}
