use minifi_native_sys::*;
use std::ffi::{c_void};
use crate::primitives::{StringView};
use crate::flowfile_wrapper::FlowFile;

pub struct SessionFactory<'a> {
    _ptr: MinifiProcessSessionFactory,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> SessionFactory<'a> {
    pub fn new(ptr: MinifiProcessSessionFactory) -> Self {
        Self {
            _ptr: ptr,
            _lifetime: std::marker::PhantomData,
        }
    }
}

/// A safe wrapper around a `MinifiProcessContext` pointer.
pub struct ProcessContext<'a> {
    ptr: MinifiProcessContext,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

unsafe extern "C" fn property_callback(output_option: *mut c_void, property_c_value: MinifiStringView) {
    let result_target = &mut *(output_option as *mut Option<String>);

    if property_c_value.data.is_null() {
        *result_target = None;
        return;
    }

    let value_slice = std::slice::from_raw_parts(property_c_value.data as *const u8, property_c_value.length as usize);
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
        property_name: &'a str,
        flow_file: Option<&FlowFile>,
    ) -> Option<String> {
        let ff_ptr = flow_file.map_or(std::ptr::null_mut(), |ff| ff.ptr);

        let mut result: Option<String> = None;
        let property_name: StringView<'a> = StringView::new(property_name);

        let status = unsafe {
            MinifiProcessContextGetProperty(
                self.ptr,
                property_name.as_raw(),
                ff_ptr,
                Some(property_callback),
                &mut result as *mut _ as *mut c_void,
            )
        };

        #[allow(non_upper_case_globals)]
        match status {
            MinifiStatus_MINIFI_SUCCESS => result,
            MinifiStatus_MINIFI_PROPERTY_NOT_SET => None,
            _ => None,
        }
    }
}
