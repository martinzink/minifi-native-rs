use std::collections::HashMap;
use crate::api::FlowFile;

pub struct MockFlowFile {
    pub content: String,
    pub attributes: HashMap<String, String>
}

impl FlowFile for MockFlowFile {
    fn set_attribute(&mut self, attribute_name: &str, attribute_value: &str) {
        self.attributes.insert(attribute_name.to_string(), attribute_value.to_string());
    }
}

impl MockFlowFile {
    pub fn new() -> MockFlowFile {
        MockFlowFile {
            content: String::new(),
            attributes: HashMap::new()
        }
    }
}