use minifi_native_sys::{MinifiInputStreamSize, MinifiInputStream, MinifiOutputStream, MinifiInputStreamRead, MinifiOutputStreamWrite, MinifiProcessSession, MinifiProcessSessionCreate, MinifiProcessSessionGet, MinifiProcessSessionTransfer, MinifiProcessSessionWrite, MinifiProcessSessionRead, MinifiStringView};
use std::ffi::{c_void, CString};

use crate::flowfile_wrapper::FlowFile;

/// A safe wrapper around a `MinifiProcessSession` pointer.
pub struct Session<'a> {
    ptr: MinifiProcessSession,
    // The lifetime ensures the session cannot outlive the `on_trigger` call.
    _lifetime: std::marker::PhantomData<&'a ()>,
}

unsafe extern "C" fn write_callback(user_ctx: *mut c_void, output_stream: MinifiOutputStream) -> i64 {
    let result_target = &mut *(user_ctx as *mut Option<&str>);
    if result_target.is_none() {
        return -1;
    }

    MinifiOutputStreamWrite(output_stream, result_target.unwrap().as_ptr() as *const i8, result_target.unwrap().len() as u64)
}

unsafe extern "C" fn read_callback(output_option: *mut c_void, input_stream: MinifiInputStream) -> i64 {
    let result_target = &mut *(output_option as *mut Option<String>);

    let stream_size = MinifiInputStreamSize(input_stream);
    let mut buffer: Vec<u8> = Vec::with_capacity(stream_size as usize);

    let bytes_read = MinifiInputStreamRead(
        input_stream,
        buffer.as_mut_ptr() as *mut i8,
        stream_size
    );

    if bytes_read < 0 {
        *result_target = None;
        return bytes_read;
    }

    buffer.set_len(bytes_read as usize);

    match String::from_utf8(buffer) {
        Ok(s) => {
            *result_target = Some(s);
            bytes_read as i64
        }
        Err(_) => {
            *result_target = None;
            -1
        }
    }
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
            Some(FlowFile { ptr: ff_ptr })
        }
    }

    /// Creates a new FlowFile. Returns 'None' if there is some problem creating the FlowFile.
    pub fn create(&mut self) -> Option<FlowFile> {
        let ff_ptr = unsafe { MinifiProcessSessionCreate(self.ptr, std::ptr::null_mut()) };
        if ff_ptr.is_null() {
            None
        } else {
            Some(FlowFile { ptr: ff_ptr })
        }
    }

    /// Transfers a FlowFile to the specified relationship.
    pub fn transfer(&mut self, flow_file: FlowFile, relationship: &str) {
        if let Ok(c_relationship) = CString::new(relationship) {  // TODO(mzink) &Cstr should be enough
            unsafe {
                MinifiProcessSessionTransfer(
                    self.ptr,
                    flow_file.ptr,
                    MinifiStringView {
                        data: c_relationship.as_ptr(),
                        length: c_relationship.as_bytes().len() as u32,
                    },
                );
            }
        }
    }

    pub fn write(&mut self, flow_file: &FlowFile, data: &str) {  // TODO(mzink) This should be Result<(), Error>
        let mut dt: Option<&str> = Some(data);
        unsafe {
            MinifiProcessSessionWrite(
                self.ptr,
                flow_file.ptr,
                Some(write_callback),
                &mut dt as *mut _ as *mut c_void,
            )
        }
    }

    pub fn read(&mut self, flow_file: &FlowFile) -> Option<String> {  // TODO(mzink) This should be Result<String, Error>
        let mut output: Option::<String> = None;
        unsafe {
            MinifiProcessSessionRead(
                self.ptr,
                flow_file.ptr,
                Some(read_callback),
                &mut output as *mut _ as *mut c_void,
            )
        }
        output
    }
}
