use super::c_ffi_flow_file::CffiFlowFile;
use super::c_ffi_primitives::StringView;
use crate::api::ProcessContext;
use crate::{MinifiError, Property};
use minifi_native_sys::*;
use std::ffi::c_void;

/// A safe wrapper around a `MinifiProcessContext` pointer.
pub struct CffiProcessContext<'a> {
    ptr: *mut MinifiProcessContext,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> CffiProcessContext<'a> {
    pub fn new(ptr: *mut MinifiProcessContext) -> Self {
        Self {
            ptr,
            _lifetime: std::marker::PhantomData,
        }
    }
}

unsafe extern "C" fn property_callback(
    output_option: *mut c_void,
    property_c_value: MinifiStringView,
) {
    unsafe {
        let result_target = &mut *(output_option as *mut Option<String>);

        if property_c_value.data.is_null() || property_c_value.length == 0 {
            *result_target = None;
            return;
        }

        let value_slice = std::slice::from_raw_parts(
            property_c_value.data as *const u8,
            property_c_value.length as usize,
        );
        if let Ok(string_value) = String::from_utf8(value_slice.to_vec()) {
            *result_target = Some(string_value);
        }
    }
}

impl<'a> ProcessContext for CffiProcessContext<'a> {
    type FlowFile = CffiFlowFile;
    fn get_property(
        &self,
        property: &Property,
        flow_file: Option<&Self::FlowFile>,
    ) -> Result<Option<String>, MinifiError> {
        let ff_ptr = flow_file.map_or(std::ptr::null_mut(), |ff| ff.ptr);

        let mut result: Option<String> = None;
        let property_name: StringView = StringView::new(property.name);

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
            MinifiStatus_MINIFI_STATUS_SUCCESS => Ok(result),
            _ => match property.is_required {
                true => Err(MinifiError::MissingRequiredProperty(property.name)),
                false => Ok(None),
            },
        }
    }
}
