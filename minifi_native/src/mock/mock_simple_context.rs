use crate::api::property::GetControllerService;
use crate::{ComponentIdentifier, EnableControllerService, GetProperty, MinifiError, Property};

pub struct MockSimpleContext {}

impl MockSimpleContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl GetProperty for MockSimpleContext {
    fn get_property(&self, _property: &Property) -> Result<Option<String>, MinifiError> {
        todo!()
    }
}

impl GetControllerService for MockSimpleContext {
    fn get_controller_service<Cs>(&self, _property: &Property) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: EnableControllerService + ComponentIdentifier + 'static,
    {
        todo!()
    }
}
