use std::collections::HashMap;
use crate::api::ProcessSession;
use crate::MockFlowFile;

pub struct MockProcessSession {
    pub input_flow_files: Vec<MockFlowFile>,
    pub transferred_flow_files: HashMap<String, MockFlowFile>,
}

impl ProcessSession for MockProcessSession {
    type FlowFile = MockFlowFile;

    fn create(&mut self) -> Option<Self::FlowFile> {
        Some(Self::FlowFile::new())
    }

    fn get(&mut self) -> Option<Self::FlowFile> {
        self.input_flow_files.pop()
    }

    fn transfer(&mut self, flow_file: Self::FlowFile, relationship: &str) {
        self.transferred_flow_files.insert(relationship.to_string(), flow_file);
    }

    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &str) {
        flow_file.content = data.to_string();
    }

    fn read(&mut self, flow_file: &Self::FlowFile) -> Option<String> {
        Some(flow_file.content.clone())
    }
}

impl MockProcessSession {
    pub fn new() -> Self {
        Self {
            transferred_flow_files: HashMap::new(),
            input_flow_files: Vec::new(),
        }
    }
}