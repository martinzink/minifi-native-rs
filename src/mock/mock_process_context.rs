use crate::api::ProcessContext;
use crate::MockFlowFile;
use std::collections::HashMap;

pub struct MockProcessContext {
    pub properties: HashMap<String, String>,
}

impl ProcessContext for MockProcessContext {
    type FlowFile = MockFlowFile;

    fn get_property(&self, property_name: &str, _flow_file: Option<&Self::FlowFile>) -> Option<String> {
        self.properties.get(property_name).cloned()
    }
}

impl MockProcessContext {
    pub fn new() -> Self {
        Self { properties: HashMap::new() }
    }
}