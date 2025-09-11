use super::c_ffi_flow_file::CffiFlowFile;
use crate::api::ProcessSession;
use minifi_native_sys::{MinifiDestroyFlowFile, MinifiFlowFileSetAttribute, MinifiFlowFileGetAttribute, MinifiInputStream, MinifiInputStreamRead, MinifiInputStreamSize, MinifiOutputStream, MinifiOutputStreamWrite, MinifiProcessSession, MinifiProcessSessionCreate, MinifiProcessSessionGet, MinifiProcessSessionRead, MinifiProcessSessionTransfer, MinifiProcessSessionWrite, MinifiStringView, MinifiFlowFileGetAttributes};
use std::ffi::{CString, c_void};
use std::os::raw::c_char;
use crate::c_ffi::c_ffi_primitives::{ConvertMinifiStringView, StringView};

pub struct CffiProcessSession<'a> {
    ptr: MinifiProcessSession,
    // The lifetime ensures the session cannot outlive the `on_trigger` call.
    _lifetime: std::marker::PhantomData<&'a ()>,
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
                MinifiDestroyFlowFile(flow_file.ptr);
            }
        }
    }

    fn set_attribute(&mut self, flow_file: &mut Self::FlowFile, attr_key: &str, attr_value: &str) {
        unsafe {
            let attr_key_string_view = StringView::new(attr_key);
            let attr_value_string_view = StringView::new(attr_value);
            MinifiFlowFileSetAttribute(
                self.ptr,
                flow_file.ptr,
                attr_key_string_view.as_raw(),
                &attr_value_string_view.as_raw()
            )
        }
    }

    fn get_attribute(&mut self, flow_file: &mut Self::FlowFile, attr_key: &str) -> Option<String> {
        let mut attr_value: Option<String> = None;
        unsafe {
            unsafe extern "C" fn cb(
                rs_attr_value: *mut c_void,
                minifi_attr_value: MinifiStringView
            ) {
                unsafe {
                    let result_target = &mut *(rs_attr_value as *mut Option<String>);
                    *result_target = minifi_attr_value.as_string().ok()
                }
            }

            let attr_key_string_view = StringView::new(attr_key);
            MinifiFlowFileGetAttribute(
                self.ptr,
                flow_file.ptr,
                attr_key_string_view.as_raw(),
                Some(cb),
                &mut attr_value as *mut _ as *mut c_void,
            );
        }
        attr_value
    }

    fn on_attributes<F: FnMut(&str, &str)>(&mut self, flow_file: &Self::FlowFile, process_attr: F) -> bool {
        struct OnAttrHelper<F: FnMut(&str, &str)> {
            result: bool,
            process_attr: F
        }

        let mut on_attr_helper = OnAttrHelper{
            result: false,
            process_attr
        };

        unsafe extern "C" fn cb<'b, F: FnMut(&str, &str)>(
            user_ctx: *mut c_void,
            minifi_attr_key: MinifiStringView,
            minifi_attr_val: MinifiStringView,
        ) {
            unsafe {
                let helper = &mut *(user_ctx as *mut OnAttrHelper<F>);
                let attr_key = minifi_attr_key.as_str();
                let attr_value = minifi_attr_val.as_str();
                if attr_key.is_err() || attr_value.is_err() {
                    helper.result = false;
                    return;  // TODO(mzink) better err handling
                }
                (helper.process_attr)(attr_key.unwrap(), attr_value.unwrap());
            }
        }

        unsafe {
            MinifiFlowFileGetAttributes(self.ptr,
                                        flow_file.ptr,
                                        Some(cb::<F>),
                                        &mut on_attr_helper as *mut _ as *mut c_void)
        }
        on_attr_helper.result
    }

    fn write(&mut self, flow_file: &mut Self::FlowFile, data: &str) {
        let mut dt: Option<&str> = Some(data);
        unsafe {
            unsafe extern "C" fn cb(
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

            match MinifiProcessSessionWrite(
                self.ptr,
                flow_file.ptr,
                Some(cb),
                &mut dt as *mut _ as *mut c_void,
            ) {
                0 => {}  // TODO(replace with const)
                _ => {}
            }
        }
    }

    fn write_in_batches<'b, F: FnMut() -> Option<&'b [u8]>>(
        &mut self,
        flow_file: &mut Self::FlowFile,
        mut produce_batch: F,
    ) -> bool {
        unsafe {
            unsafe extern "C" fn cb<'b, F: FnMut() -> Option<&'b [u8]>>(
                user_ctx: *mut c_void,
                output_stream: MinifiOutputStream,
            ) -> i64 {
                unsafe {
                    let produce_batch = &mut *(user_ctx as *mut F);
                    let mut overall_writes = 0;
                    while let Some(batch) = produce_batch() {
                        overall_writes += MinifiOutputStreamWrite(
                            output_stream,
                            batch.as_ptr() as *const c_char,
                            batch.len() as u64,
                        )
                    }
                    overall_writes
                }
            }

            match MinifiProcessSessionWrite(
                self.ptr,
                flow_file.ptr,
                Some(cb::<F>),
                &mut produce_batch as *mut _ as *mut c_void,
            ) {
                0 => { true }  // TODO(replace with const)
                _ => { false }
            }
        }
    }

    fn read_as_string(&mut self, flow_file: &Self::FlowFile) -> Option<String> {
        let mut output: Option<String> = None;
        unsafe {
            unsafe extern "C" fn cb(
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

                    let bytes_read = MinifiInputStreamRead(
                        input_stream,
                        buffer.as_mut_ptr() as *mut c_char,
                        stream_size,
                    );

                    if bytes_read < 0 {
                        *result_target = None;
                        return bytes_read;
                    }

                    buffer.set_len(bytes_read as usize);

                    match String::from_utf8(buffer) {
                        Ok(s) => {
                            *result_target = Some(s);
                            bytes_read
                        }
                        Err(_) => {
                            *result_target = None;
                            0
                        }
                    }
                }
            }

            MinifiProcessSessionRead(
                self.ptr,
                flow_file.ptr,
                Some(cb),
                &mut output as *mut _ as *mut c_void,
            );
        }
        output
    }

    fn read_in_batches<F: FnMut(&[u8])>(
        &mut self,
        flow_file: &Self::FlowFile,
        batch_size: usize,
        process_batch: F,
    ) -> bool {
        struct BatchReadHelper<F: FnMut(&[u8])> {
            batch_size: usize,
            process_batch: F,
        }

        let mut batch_helper = BatchReadHelper {
            batch_size,
            process_batch,
        };
        unsafe {
            unsafe extern "C" fn cb<F: FnMut(&[u8])>(
                output_option: *mut c_void,
                input_stream: MinifiInputStream,
            ) -> i64 {
                unsafe {
                    let batch_helper = &mut *(output_option as *mut BatchReadHelper<F>);

                    let mut remaining_size = MinifiInputStreamSize(input_stream) as usize;
                    let mut overall_read = 0;
                    while remaining_size > 0 {
                        let read_size = remaining_size.min(batch_helper.batch_size);
                        let mut buffer: Vec<u8> = Vec::with_capacity(read_size);

                        let bytes_read = MinifiInputStreamRead(
                            input_stream,
                            buffer.as_mut_ptr() as *mut c_char,
                            read_size as u64,
                        );
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

            MinifiProcessSessionRead(
                self.ptr,
                flow_file.ptr,
                Some(cb::<F>),
                &mut batch_helper as *mut _ as *mut c_void,
            );
        }
        true
    }
}
