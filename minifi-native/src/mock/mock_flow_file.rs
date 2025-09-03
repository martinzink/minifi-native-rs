use crate::api::FlowFile;

pub struct MockFlowFile {
    pub content: String
}

impl FlowFile for MockFlowFile {}

impl MockFlowFile {
    pub fn new() -> MockFlowFile {
        MockFlowFile {
            content: String::new()
        }
    }
}