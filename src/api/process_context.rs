use crate::api::flow_file::FlowFile;

pub trait ProcessContext {
    type FlowFile: FlowFile;

    fn get_property(
        &self,
        property_name: &str,
        flow_file: Option<&Self::FlowFile>,
    ) -> Option<String>;
}
