use minifi_native_sys::MinifiFlowFile;
use crate::api::FlowFile;

pub struct CffiFlowFile {
    pub ptr : MinifiFlowFile,
}

impl FlowFile for CffiFlowFile {

}
