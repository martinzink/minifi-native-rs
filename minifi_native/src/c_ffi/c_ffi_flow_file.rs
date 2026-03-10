use crate::api::FlowFile;
use minifi_native_sys::MinifiFlowFile;

pub struct CffiFlowFile<'a> {
    pub(crate) ptr: *mut MinifiFlowFile,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl CffiFlowFile<'_> {
    pub(crate) fn new(ptr: *mut MinifiFlowFile) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }
}

impl FlowFile for CffiFlowFile<'_> {}
