use minifi_native_sys::{MinifiDestroyFlowFile, MinifiInputStream, MinifiInputStreamRead, MinifiInputStreamSize, MinifiOutputStream, MinifiOutputStreamWrite, MinifiProcessSession, MinifiProcessSessionCreate, MinifiProcessSessionGet, MinifiProcessSessionRead, MinifiProcessSessionTransfer, MinifiProcessSessionWrite, MinifiStringView};
use std::ffi::{c_void, CString};
use std::os::raw::c_char;
use super::c_ffi_flow_file::CffiFlowFile;
use crate::api::ProcessSession;

pub struct CffiProcessSession<'a> {
    ptr: MinifiProcessSession,
    // The lifetime ensures the session cannot outlive the `on_trigger` call.
    _lifetime: std::marker::PhantomData<&'a ()>,
}

unsafe extern "C" fn write_callback(
    user_ctx: *mut c_void,
    output_stream: MinifiOutputStream,
) -> i64 {
    unsafe {
        let result_target = &mut *(user_ctx as *mut Option<&str>);
        if result_target.is_none() {
            return -1;
        }

        MinifiOutputStreamWrite(
            output_stream,
            result_target.unwrap().as_ptr() as *const c_char,
            result_target.unwrap().len() as u64,
        )
    }
}

unsafe extern "C" fn read_as_string_callback(
    output_option: *mut c_void,
    input_stream: MinifiInputStream,
) -> i64 {
    unsafe {
        let result_target = &mut *(output_option as *mut Option<String>);

        let stream_size = MinifiInputStreamSize(input_stream);
        if stream_size == 0 {
            *result_target = None;
            return 0;
        }
        let mut buffer: Vec<u8> = Vec::with_capacity(stream_size as usize);

        let bytes_read =
            MinifiInputStreamRead(input_stream, buffer.as_mut_ptr() as *mut c_char, stream_size);

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
                0
            }
        }
    }
}

struct BatchReadHelper<F: FnMut(&[u8])> {
    batch_size: usize,
    process_batch: F
}

unsafe extern "C" fn read_batch_callback<F: FnMut(&[u8])>(
    output_option: *mut c_void,
    input_stream: MinifiInputStream
) -> i64 {
    unsafe {
        let batch_helper = &mut *(output_option as *mut BatchReadHelper<F>);

        let mut remaining_size = MinifiInputStreamSize(input_stream) as usize;
        let mut overall_read = 0;
        while remaining_size > 0 {
            let read_size = remaining_size.min(batch_helper.batch_size);
            let mut buffer: Vec<u8> = Vec::with_capacity(read_size);

            let bytes_read = MinifiInputStreamRead(input_stream, buffer.as_mut_ptr() as *mut c_char, read_size as u64);
            if bytes_read < 0 || bytes_read > read_size as i64 {
                return -1;
            }

            buffer.set_len(bytes_read as usize);

            (batch_helper.process_batch)(&*buffer);
            remaining_size -= bytes_read as usize;
            overall_read += bytes_read;
        }
        overall_read
    }
}

impl<'a> CffiProcessSession<'a> {
    pub fn new(ptr: MinifiProcessSession) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }
}



impl<'a> ProcessSession for CffiProcessSession<'a> {
    type FlowFile = CffiFlowFile;

    fn create(&mut self) -> Option<Self::FlowFile> {
        let ff_ptr = unsafe { MinifiProcessSessionCreate(self.ptr, std::ptr::null_mut()) };
        if ff_ptr.is_null() {
            None
        } else {
            Some(Self::FlowFile { ptr: ff_ptr })
        }
    }

    fn get(&mut self) -> Option<Self::FlowFile> {
        let ff_ptr = unsafe { MinifiProcessSessionGet(self.ptr) };
        if ff_ptr.is_null() {
            None
        } else {
            Some(Self::FlowFile { ptr: ff_ptr })
        }
    }

    fn transfer(&mut self, flow_file: Self::FlowFile, relationship: &str) {
        if let Ok(c_relationship) = CString::new(relationship) {
            unsafe {
                MinifiProcessSessionTransfer(
                    self.ptr,
                    flow_file.ptr,
                    MinifiStringView {
                        data: c_relationship.as_ptr(),
                        length: c_relationship.as_bytes().len() as u32,
                    },
                );
                MinifiDestroyFlowFile(
                    flow_file.ptr,
                );
            }
        }
    }

    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &str) {
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

    fn read_as_string(&mut self, flow_file: &Self::FlowFile) -> Option<String> {
        let mut output: Option<String> = None;
        unsafe {
            MinifiProcessSessionRead(
                self.ptr,
                flow_file.ptr,
                Some(read_as_string_callback),
                &mut output as *mut _ as *mut c_void,
            )
        }
        output
    }

    fn read_in_batches<F: FnMut(&[u8])>(&mut self, flow_file: &Self::FlowFile, batch_size: usize, process_batch: F) -> bool {
        let mut batch_helper = BatchReadHelper{ batch_size, process_batch };
        unsafe {
            MinifiProcessSessionRead(
                self.ptr,
                flow_file.ptr,
                Some(read_batch_callback::<F>),
                &mut batch_helper as *mut _ as *mut c_void,
            )
        }
        true
    }
}
