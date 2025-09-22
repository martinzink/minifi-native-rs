use crate::api::flow_file::FlowFile;
use crate::MinifiError;

pub trait ProcessSession {
    type FlowFile: FlowFile;

    fn create(&mut self) -> Result<Self::FlowFile, MinifiError>;
    fn get(&mut self) -> Option<Self::FlowFile>;
    fn transfer(&mut self, flow_file: Self::FlowFile, relationship: &str);

    fn set_attribute(&mut self, flow_file: &mut Self::FlowFile, attr_key: &str, attr_value: &str);  // TODO(mzink) Result
    fn get_attribute(&mut self, flow_file: &mut Self::FlowFile, attr_key: &str) -> Option<String>;  // TODO(mzink) Result
    fn on_attributes<F: FnMut(&str, &str)>(&mut self, flow_file: &Self::FlowFile, process_attr: F) -> bool; // TODO(mzink) Result

    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &[u8]);
    fn write_in_batches<'b, F: FnMut() -> Option<&'b [u8]>>(
        &mut self,
        flow_file: &mut Self::FlowFile,
        produce_batch: F,
    ) -> bool;
    fn read(&mut self, flow_file: &Self::FlowFile) -> Option<Vec<u8>>;
    fn read_in_batches<F: FnMut(&[u8])>(
        &mut self,
        flow_file: &Self::FlowFile,
        batch_size: usize,
        process_batch: F,
    ) -> bool;
}
