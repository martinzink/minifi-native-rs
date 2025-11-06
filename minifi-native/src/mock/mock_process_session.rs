use crate::api::ProcessSession;
use crate::{MinifiError, MockFlowFile};
use itertools::Itertools;

pub struct TransferredFlowFile {
    pub relationship: String,
    pub flow_file: MockFlowFile,
}

pub struct MockProcessSession {
    pub input_flow_files: Vec<MockFlowFile>,
    pub transferred_flow_files: Vec<TransferredFlowFile>,
}

impl ProcessSession for MockProcessSession {
    type FlowFile = MockFlowFile;

    fn create(&mut self) -> Result<Self::FlowFile, MinifiError> {
        Ok(Self::FlowFile::new())
    }
    fn get(&mut self) -> Option<Self::FlowFile> {
        self.input_flow_files.pop()
    }
    fn transfer(&mut self, flow_file: Self::FlowFile, relationship: &str) {
        self.transferred_flow_files.push(TransferredFlowFile {
            relationship: relationship.to_string(),
            flow_file,
        });
    }

    fn set_attribute(&mut self, flow_file: &mut Self::FlowFile, attr_key: &str, attr_value: &str) {
        flow_file
            .attributes
            .insert(attr_key.to_string(), attr_value.to_string());
    }
    fn get_attribute(&self, flow_file: &mut Self::FlowFile, attr_key: &str) -> Option<String> {
        flow_file.attributes.get(attr_key).cloned()
    }

    fn on_attributes<F: FnMut(&str, &str)>(
        &self,
        flow_file: &Self::FlowFile,
        mut process_attr: F,
    ) -> bool {
        // Sorting for deterministic tests.
        for (attr_key, attr_value) in flow_file.attributes.iter().sorted_by_key(|x| x.0) {
            process_attr(attr_key, attr_value);
        }
        true
    }

    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &[u8]) {
        flow_file.content = data.to_vec();
    }

    fn write_in_batches<'b, F: FnMut() -> Option<&'b [u8]>>(
        &mut self,
        flow_file: &mut Self::FlowFile,
        mut produce_batch: F,
    ) -> bool {
        flow_file.content.clear();
        while let Some(batch) = produce_batch() {
            flow_file.content.append(&mut batch.to_vec());
        }
        true
    }

    fn read(&mut self, flow_file: &Self::FlowFile) -> Option<Vec<u8>> {
        Some(flow_file.content.to_vec())
    }

    fn read_in_batches<F>(
        &mut self,
        flow_file: &Self::FlowFile,
        batch_size: usize,
        mut process_batch: F,
    ) -> Result<(), MinifiError>
    where
        F: FnMut(&[u8]) -> Result<(), MinifiError>,
    {
        for chunk in flow_file.content.chunks(batch_size) {
            process_batch(chunk)?;
        }
        Ok(())
    }
}

impl MockProcessSession {
    pub fn new() -> Self {
        Self {
            transferred_flow_files: Vec::new(),
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
        flow_file.content = "Hello, World!".to_string().as_bytes().to_vec();
        let mut vec: Vec<u8> = Vec::new();

        let res =session.read_in_batches(&mut flow_file, 1, |batch| {
            assert_eq!(batch.len(), 1);
            vec.push(batch[0]);
            Ok(())
        });

        assert_eq!(res, Ok(()));

        assert_eq!(vec.len(), 13);
        assert_eq!(vec, b"Hello, World!");
    }
}
