use crate::api::ProcessContext;
use crate::{ControllerService, MinifiError, MockFlowFile, Property};
use std::collections::HashMap;

pub struct MockProcessContext {
    pub properties: HashMap<String, String>,
}

impl ProcessContext for MockProcessContext {
    type FlowFile = MockFlowFile;

    fn get_property(
        &self,
        property: &Property,
        _flow_file: Option<&Self::FlowFile>,
    ) -> Result<Option<String>, MinifiError> {
        if let Some(property) = self.properties.get(property.name) {
            Ok(Some(property.clone()))
        } else {
            if let Some(default_val) = property.default_value {
                return Ok(Some(default_val.to_string()));
            }
            match property.is_required {
                true => Err(MinifiError::MissingRequiredProperty(property.name)),
                false => Ok(None),
            }
        }
    }

    fn get_controller_service<Cs>(&self, _property: &Property) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: ControllerService
    {
        Ok(None)
    }
}

impl MockProcessContext {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
}
