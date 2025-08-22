use minifi_native_sys::*;
use std::ffi::{c_void, CString};
pub(crate) use crate::relationship_wrapper::Relationship;
pub(crate) use crate::property_wrapper::Property;
pub(crate) use crate::primitives::create_string_view;

/// A safe wrapper around a `MinifiLogger` pointer.
#[derive(Clone, Copy)]
pub struct Logger(MinifiLogger);

impl Logger {
    pub fn new(logger: MinifiLogger) -> Self {
        Self(logger)
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if let Ok(c_message) = CString::new(message) {
            unsafe {
                MinifiLoggerLogString(
                    self.0,
                    level.into(),
                    MinifiStringView {
                        data: c_message.as_ptr(),
                        length: c_message.as_bytes().len() as u32,
                    },
                );
            }
        }
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
}

/// Represents the log level for a message.
pub enum LogLevel {
    Info,
    // Add other levels (Debug, Warn, Error) as needed
}

impl From<LogLevel> for MinifiLogLevel {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Info => MinifiLogLevel_MINIFI_INFO,
        }
    }
}

/// A safe wrapper around a `MinifiFlowFile` pointer.
pub struct FlowFile(MinifiFlowFile);

/// A safe wrapper around a `MinifiProcessSession` pointer.
pub struct Session<'a> {
    ptr: MinifiProcessSession,
    // The lifetime ensures the session cannot outlive the `on_trigger` call.
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> Session<'a> {
    pub fn new(ptr: MinifiProcessSession) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }

    /// Gets the next FlowFile from the input queue. Returns `None` if the queue is empty.
    pub fn get(&mut self) -> Option<FlowFile> {
        let ff_ptr = unsafe { MinifiProcessSessionGet(self.ptr) };
        if ff_ptr.is_null() {
            None
        } else {
            Some(FlowFile(ff_ptr))
        }
    }

    /// Transfers a FlowFile to the specified relationship.
    pub fn transfer(&mut self, flow_file: FlowFile, relationship: &str) {
        if let Ok(c_relationship) = CString::new(relationship) {
            unsafe {
                MinifiProcessSessionTransfer(
                    self.ptr,
                    flow_file.0,
                    MinifiStringView {
                        data: c_relationship.as_ptr(),
                        length: c_relationship.as_bytes().len() as u32,
                    },
                );
            }
        }
    }
}

pub struct SessionFactory<'a> {
    ptr: MinifiProcessSessionFactory,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> SessionFactory<'a> {
    pub fn new(ptr: MinifiProcessSessionFactory) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }
}

/// A safe wrapper around a `MinifiProcessorDescriptor` pointer.
pub struct Descriptor<'a> {
    ptr: MinifiProcessorDescriptor,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> Descriptor<'a> {
    pub fn new(ptr: MinifiProcessorDescriptor) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }

    pub fn set_supported_relationships(&mut self, relationships: &[Relationship]) {
        let c_relationships: Vec<MinifiRelationship> =
            relationships.iter().map(|r| r.c_struct).collect();
        unsafe {
            MinifiProcessorDescriptorSetSupportedRelationships(
                self.ptr,
                c_relationships.len() as u32,
                c_relationships.as_ptr(),
            );
        }
    }

    pub fn set_supported_properties(&mut self, properties: &[Property]) {
        let c_properties: Vec<MinifiProperty> =
            properties.iter().map(|p| p.c_struct).collect();
        unsafe {
            MinifiProcessorDescriptorSetSupportedProperties(
                self.ptr,
                c_properties.len() as u32,
                c_properties.as_ptr(),
            );
        }
    }
}

/// A safe wrapper around a `MinifiProcessContext` pointer.
pub struct ProcessContext<'a> {
    ptr: MinifiProcessContext,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

unsafe extern "C" fn property_callback(data: *mut c_void, result_sv: MinifiStringView) {
    let result_target = &mut *(data as *mut Option<String>);

    if result_sv.data.is_null() {
        *result_target = None;
        return;
    }

    let value_slice = std::slice::from_raw_parts(result_sv.data as *const u8, result_sv.length as usize);
    if let Ok(string_value) = String::from_utf8(value_slice.to_vec()) {
        *result_target = Some(string_value);
    }
}

impl<'a> ProcessContext<'a> {
    pub fn new(ptr: MinifiProcessContext) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }

    pub fn get_property(
        &self,
        property: &Property,
        flow_file: Option<&FlowFile>,
    ) -> Option<String> {
        let ff_ptr = flow_file.map_or(std::ptr::null_mut(), |ff| ff.0);

        let mut result: Option<String> = None;

        let status = unsafe {
            MinifiProcessContextGetProperty(
                self.ptr,
                property.c_struct.name,
                ff_ptr,
                Some(property_callback),
                &mut result as *mut _ as *mut c_void,
            )
        };

        match status {
            MinifiStatus_MINIFI_SUCCESS => result,
            MinifiStatus_MINIFI_PROPERTY_NOT_SET => None,
            _ => None,
        }
    }
}
