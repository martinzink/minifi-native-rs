use crate::api::ProcessContext;
use crate::{
    ComponentIdentifier, EnableControllerService, MinifiError, MockFlowFile, Property,
    RawControllerService,
};
use std::any::Any;
use std::collections::HashMap;

pub struct MockPropertyMap {
    pub properties: HashMap<String, String>,
}

impl MockPropertyMap {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.properties.insert(key.into(), value.into());
    }

    pub fn extend<I, K, V>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.properties
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())))
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

    fn get_raw_controller_service<Cs>(
        &self,
        _property: &Property,
    ) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: RawControllerService + ComponentIdentifier + 'static,
    {
        panic!("Not implemented yet");
    }

    fn get_controller_service<Cs>(&self, property: &Property) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: EnableControllerService + ComponentIdentifier + 'static,
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
