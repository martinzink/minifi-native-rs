use crate::api::flow_file::FlowFile;

pub trait ProcessSession {
    type FlowFile: FlowFile;

    fn create(&mut self) -> Option<Self::FlowFile>;
    fn get(&mut self) -> Option<Self::FlowFile>;
    fn transfer(&mut self, flow_file: Self::FlowFile, relationship: &str);
    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &str);
    fn read_as_string(&mut self, flow_file: &Self::FlowFile) -> Option<String>;
    fn read_in_batches<F: FnMut(&[u8])>(&mut self, flow_file: &Self::FlowFile, batch_size: usize, process_batch: F) -> bool;
}
