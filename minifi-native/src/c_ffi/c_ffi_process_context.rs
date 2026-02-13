use super::c_ffi_flow_file::CffiFlowFile;
use super::c_ffi_primitives::{ConvertMinifiStringView, FfiConversionError, StringView};
use crate::api::ProcessContext;
use crate::{ComponentIdentifier, MinifiError, Property, RawControllerService};
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

unsafe extern "C" fn get_property_callback(
    output_option: *mut c_void,
    property_c_value: MinifiStringView,
) {
    unsafe {
        let result_target = &mut *(output_option as *mut Option<String>);

        if property_c_value.data.is_null() || property_c_value.length == 0 {
            *result_target = None;
            return;
        }

        let value_slice =
            std::slice::from_raw_parts(property_c_value.data as *const u8, property_c_value.length);
        if let Ok(string_value) = String::from_utf8(value_slice.to_vec()) {
            *result_target = Some(string_value);
        }
    }
}

#[derive(Debug)]
struct ControllerServiceHelper {
    result: Option<*mut c_void>,
    class_name_str: &'static str,
    group_name_str: &'static str,
    version_str: &'static str,
}

impl ControllerServiceHelper {
    fn is_valid(
        &self,
        class: &MinifiStringView,
        grp: &MinifiStringView,
        version: &MinifiStringView,
    ) -> Result<bool, FfiConversionError> {
        unsafe {
            Ok(self
                .class_name_str
                .ends_with(class.as_str()?)
                && self.group_name_str == grp.as_str()?
                && self.version_str == version.as_str()?)
        }
    }
}

unsafe extern "C" fn get_controller_service_callback(
    controller_service_helper_ptr: *mut c_void,
    controller_ptr: *mut c_void,
    class_name: MinifiStringView,
    group_name: MinifiStringView,
    version: MinifiStringView,
) -> MinifiStatus {
    unsafe {
        let controller_service_helper =
            &mut *(controller_service_helper_ptr as *mut ControllerServiceHelper);

        // TODO(mzink) maybe do some logging?
        match controller_service_helper.is_valid(&class_name, &group_name, &version) {
            Ok(false) => MinifiStatus_MINIFI_STATUS_VALIDATION_FAILED,
            Ok(true) => {
                controller_service_helper.result = Some(controller_ptr);
                MinifiStatus_MINIFI_STATUS_SUCCESS
            },
            Err(_e) => MinifiStatus_MINIFI_STATUS_UNKNOWN_ERROR
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
                Some(get_property_callback),
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

    fn get_controller_service<Cs>(&self, property: &Property) -> Result<Option<&Cs>, MinifiError>
    where
        Cs: RawControllerService + ComponentIdentifier,
    {
        if let Some(service_name) = self.get_property(property, None)? {
            let str_view = StringView::new(service_name.as_str());
            let mut helper = ControllerServiceHelper {
                result: None,
                class_name_str: Cs::CLASS_NAME,
                group_name_str: Cs::GROUP_NAME,
                version_str: Cs::VERSION,
            };
            unsafe {
                MinifiProcessContextGetControllerService(
                    self.ptr,
                    str_view.as_raw(),
                    Some(get_controller_service_callback),
                    &mut helper as *mut _ as *mut c_void,
                );
            }

            match helper.result {
                Some(result) => {
                    let foo_ref: &Cs = unsafe {
                        (result as *const Cs)
                            .as_ref()
                            .expect("C returned a null pointer")
                    };
                    Ok(Some(foo_ref))
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
