use crate::api::ProcessContext;
use crate::{ControllerService, MinifiError, MockFlowFile, Property};
use std::any::Any;
use std::collections::HashMap;

pub struct MockPropertyMap {
    properties: HashMap<String, String>,
}

impl MockPropertyMap {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
}

impl MockPropertyMap {
    pub fn get_property(
        &self,
        property: &Property,
        _flow_file: Option<&MockFlowFile>,
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
}

pub struct MockProcessContext {
    pub properties: MockPropertyMap,
    pub controller_services: HashMap<String, Box<dyn Any>>,
}

impl ProcessContext for MockProcessContext {
    type FlowFile = MockFlowFile;

    fn get_property(
        &self,
        property: &Property,
        _flow_file: Option<&Self::FlowFile>,
    ) -> Result<Option<String>, MinifiError> {
        self.properties.get_property(property, _flow_file)
    }

    fn get_controller_service<Cs>(&self, property: &Property) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: ControllerService + 'static,
    {
        if let Some(service_name) = self.get_property(property, None)? {
            Ok(self
                .controller_services
                .get(&service_name)
                .and_then(|c| c.downcast_ref::<Cs>()))
        } else {
            Ok(None)
        }
    }
}

impl MockProcessContext {
    pub fn new() -> Self {
        Self {
            properties: MockPropertyMap::new(),
            controller_services: HashMap::new(),
        }
    }
}
