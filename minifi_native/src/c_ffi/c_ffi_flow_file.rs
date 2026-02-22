use crate::api::FlowFile;
use minifi_native_sys::MinifiFlowFile;

pub struct CffiFlowFile {
    pub ptr: *mut MinifiFlowFile,
}

impl FlowFile for CffiFlowFile {}
