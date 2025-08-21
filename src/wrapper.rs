// minifi/src/wrapper.rs

//! Safe Rust wrappers around the raw C API types from `minifi-sys`.

use minificpp_sys::*;
use std::ffi::CString;

pub struct Relationship {
    // This holds the C-compatible struct internally.
    c_struct: MinifiRelationship,
}

impl Relationship {
    /// Creates a new Relationship with a name and a description.
    pub fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            c_struct: MinifiRelationship {
                name: MinifiStringView {
                    data: name.as_ptr() as *const i8,
                    length: name.len() as u32,
                },
                description: MinifiStringView {
                    data: description.as_ptr() as *const i8,
                    length: description.len() as u32,
                },
            },
        }
    }
}

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

    /// Sets the supported relationships for the processor using the new safe `Relationship` struct.
    pub fn set_supported_relationships(&mut self, relationships: &[Relationship]) {
        // Convert the slice of safe Rust structs to a slice of the raw C structs.
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
}

/// A safe wrapper around a `MinifiProcessContext` pointer.
pub struct ProcessContext<'a> {
    ptr: MinifiProcessContext,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> ProcessContext<'a> {
    pub fn new(ptr: MinifiProcessContext) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }
    // Add safe methods to interact with the context here, e.g., get_property
}
