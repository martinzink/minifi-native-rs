use crate::MinifiError;
use crate::api::flow_file::FlowFile;
use std::io::Read;

pub trait ProcessSession {
    type FlowFile: FlowFile;

    fn create(&mut self) -> Result<Self::FlowFile, MinifiError>;
    fn get(&mut self) -> Option<Self::FlowFile>;
    fn transfer(
        &self,
        flow_file: Self::FlowFile,
        relationship: &str,
    ) -> Result<(), MinifiError>;
    fn remove(&mut self, flow_file: Self::FlowFile) -> Result<(), MinifiError>;

    fn set_attribute(
        &self,
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

    fn write(&self, flow_file: &Self::FlowFile, data: &[u8]) -> Result<(), MinifiError>;
    fn write_stream<'a>(
        &self,
        flow_file: &Self::FlowFile,
        stream: Box<dyn Read + 'a>,
    ) -> Result<(), MinifiError>;

    fn read(&self, flow_file: &Self::FlowFile) -> Option<Vec<u8>>;
    fn read_stream<F, R>(&self, flow_file: &Self::FlowFile, callback: F) -> Result<R, MinifiError>
    where
        F: FnOnce(&mut dyn std::io::Read, &Self::FlowFile) -> Result<R, MinifiError>;
    fn read_in_batches<F>(
        &self,
        flow_file: &Self::FlowFile,
        batch_size: usize,
        process_batch: F,
    ) -> Result<(), MinifiError>
    where
        F: FnMut(&[u8]) -> Result<(), MinifiError>;
}
