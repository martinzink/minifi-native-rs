use crate::api::ControllerServiceContext;
use crate::{MinifiError, Property};
use crate::mock::mock_process_context::MockPropertyMap;

pub struct MockControllerServiceContext {
    pub properties: MockPropertyMap,
}

impl ControllerServiceContext for MockControllerServiceContext {
    fn get_property(&self, property: &Property) -> Result<Option<String>, MinifiError> {
        self.properties.get_property(property, None)
    }
}

impl MockControllerServiceContext {
    pub fn new() -> Self {
        Self {
            properties: MockPropertyMap::new(),
        }
    }
}
