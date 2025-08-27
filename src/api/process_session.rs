use crate::api::flow_file::FlowFile;

pub trait ProcessSession {
    type FlowFile: FlowFile;

    fn create(&mut self) -> Option<Self::FlowFile>;
    fn get(&mut self) -> Option<Self::FlowFile>;
    fn transfer(&mut self, flow_file: Self::FlowFile, relationship: &str);
    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &str);
    fn read(&mut self, flow_file: &Self::FlowFile) -> Option<String>;
}
