use crate::api::ControllerServiceContext;
use crate::{MinifiError, Property};
use std::collections::HashMap;

pub struct MockControllerServiceContext {
    pub properties: HashMap<String, String>,
}

impl ControllerServiceContext for MockControllerServiceContext {
    fn get_property(&self, property: &Property) -> Result<Option<String>, MinifiError> {
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
}

impl MockControllerServiceContext {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
}
