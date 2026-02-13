use crate::api::FlowFile;
use std::collections::HashMap;

pub struct MockFlowFile {
    pub content: Vec<u8>,
    pub attributes: HashMap<String, String>,
}

impl FlowFile for MockFlowFile {}

impl MockFlowFile {
    pub fn new() -> MockFlowFile {
        MockFlowFile {
            content: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn with_content(content: Vec<u8>) -> MockFlowFile {
        Self {
            content,
            attributes: HashMap::new(),
        }
    }
}
