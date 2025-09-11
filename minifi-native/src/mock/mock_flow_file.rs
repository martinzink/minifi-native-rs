use crate::api::FlowFile;
use std::collections::HashMap;

pub struct MockFlowFile {
    pub content: String,
    pub attributes: HashMap<String, String>,
}

impl FlowFile for MockFlowFile {

}

impl MockFlowFile {
    pub fn new() -> MockFlowFile {
        MockFlowFile {
            content: String::new(),
            attributes: HashMap::new(),
        }
    }
}
