use minifi_native_sys::MinifiFlowFile;

/// A safe wrapper around a `MinifiFlowFile` pointer.
pub struct FlowFile {
    pub ptr : MinifiFlowFile,
}