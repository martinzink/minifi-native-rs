mod properties;

use crate::controller_services::dummy_controller_service::properties::DATA;
use minifi_native::MinifiError::MissingRequiredProperty;
use minifi_native::macros::ComponentIdentifier;
use minifi_native::{
    ControllerServiceContext, ControllerServiceDefinition, EnableControllerService, Logger,
    MinifiError, Property,
};

#[derive(Debug, ComponentIdentifier)]
pub(crate) struct DummyControllerService {
    pub data: String,
}

impl EnableControllerService for DummyControllerService {
    fn enable<P: ControllerServiceContext, L: Logger>(
        context: &P,
        _logger: &L,
    ) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        match context.get_property(&DATA)? {
            None => Err(MissingRequiredProperty("Missing data")),
            Some(data) => Ok(Self { data }),
        }
    }
}

impl ControllerServiceDefinition for DummyControllerService {
    const DESCRIPTION: &'static str = "Simple Rusty Controller Service to test API";
    const PROPERTIES: &'static [Property] = &[DATA];
}
