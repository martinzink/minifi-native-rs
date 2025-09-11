use crate::{MockFlowFile, Property};
use crate::api::ProcessContext;
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
    ) -> Option<String> {
        self.properties.get(property.name).cloned()
    }
}

impl MockProcessContext {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
}
