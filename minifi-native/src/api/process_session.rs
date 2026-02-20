use crate::MinifiError;
use crate::api::flow_file::FlowFile;
use std::io::Read;

pub trait ProcessSession {
    type FlowFile: FlowFile;

    fn create(&mut self) -> Result<Self::FlowFile, MinifiError>;
    fn get(&mut self) -> Option<Self::FlowFile>;
    fn transfer(
        &mut self,
        flow_file: Self::FlowFile,
        relationship: &str,
    ) -> Result<(), MinifiError>;
    fn remove(&mut self, flow_file: Self::FlowFile) -> Result<(), MinifiError>;

    fn set_attribute(
        &mut self,
        flow_file: &mut Self::FlowFile,
        attr_key: &str,
        attr_value: &str,
    ) -> Result<(), MinifiError>;
    fn get_attribute(&self, flow_file: &mut Self::FlowFile, attr_key: &str) -> Option<String>;
    fn on_attributes<F: FnMut(&str, &str)>(
        &self,
        flow_file: &Self::FlowFile,
        process_attr: F,
    ) -> bool;

    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &[u8]) -> Result<(), MinifiError>;
    fn write_stream<'a>(
        &mut self,
        flow_file: &mut Self::FlowFile,
        stream: Box<dyn Read + 'a>,
    ) -> Result<(), MinifiError>;

    fn read(&mut self, flow_file: &Self::FlowFile) -> Option<Vec<u8>>;
    fn read_in_batches<F>(
        &mut self,
        flow_file: &Self::FlowFile,
        batch_size: usize,
        process_batch: F,
    ) -> Result<(), MinifiError>
    where
        F: FnMut(&[u8]) -> Result<(), MinifiError>;
}
