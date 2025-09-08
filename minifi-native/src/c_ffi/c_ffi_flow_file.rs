use crate::api::FlowFile;
use minifi_native_sys::{MinifiFlowFile, MinifiFlowFileSetAttribute};
use crate::c_ffi::c_ffi_primitives::StringView;

pub struct CffiFlowFile {
    pub ptr: MinifiFlowFile,
}

impl FlowFile for CffiFlowFile {
    fn set_attribute(&mut self, attribute_name: &str, attribute_value: &str) {
        unsafe {
            let minifi_attr_name = StringView::new(attribute_name);
            let minifi_attr_value = StringView::new(attribute_value);
            MinifiFlowFileSetAttribute(self.ptr, minifi_attr_name.as_raw(), minifi_attr_value.as_raw())
        }
    }
}
