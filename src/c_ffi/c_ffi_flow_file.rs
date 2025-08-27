use crate::api::FlowFile;
use minifi_native_sys::MinifiFlowFile;

pub struct CffiFlowFile {
    pub ptr: MinifiFlowFile,
}

impl FlowFile for CffiFlowFile {}
