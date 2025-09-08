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

    fn read_as_string(&mut self, flow_file: &Self::FlowFile) -> Option<String> {
        Some(flow_file.content.clone())
    }

    fn read_in_batches<F: FnMut(&[u8])>(&mut self, flow_file: &Self::FlowFile, batch_size: usize, mut process_batch: F) -> bool {
        let bytes = flow_file.content.as_bytes();
        for chunk in bytes.chunks(batch_size) {
            process_batch(chunk);
        }
        true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_in_batches() {
        let mut session = MockProcessSession::new();
        let mut flow_file = MockFlowFile::new();
        flow_file.content = "Hello, World!".to_string();
        let mut vec : Vec<u8> = Vec::new();

        session.read_in_batches(&mut flow_file, 1, |batch| {
            assert_eq!(batch.len(), 1);
            vec.push(batch[0]);
        });

        assert_eq!(vec.len(), 13);
        assert_eq!(vec, b"Hello, World!");
    }
}