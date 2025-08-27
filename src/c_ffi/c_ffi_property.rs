use super::c_ffi_primitives::{BoolAsMinifiCBool, StaticStrAsMinifiCStr};
use crate::{Property, StandardPropertyValidator};
use minifi_native_sys::{
    MinifiGetStandardValidator, MinifiProperty, MinifiPropertyValidator,
    MinifiStandardPropertyValidator, MinifiStandardPropertyValidator_MINIFI_ALWAYS_VALID_VALIDATOR,
    MinifiStandardPropertyValidator_MINIFI_BOOLEAN_VALIDATOR,
    MinifiStandardPropertyValidator_MINIFI_DATA_SIZE_VALIDATOR,
    MinifiStandardPropertyValidator_MINIFI_INTEGER_VALIDATOR,
    MinifiStandardPropertyValidator_MINIFI_NON_BLANK_VALIDATOR,
    MinifiStandardPropertyValidator_MINIFI_PORT_VALIDATOR,
    MinifiStandardPropertyValidator_MINIFI_TIME_PERIOD_VALIDATOR,
    MinifiStandardPropertyValidator_MINIFI_UNSIGNED_INTEGER_VALIDATOR, MinifiStringView,
};
use std::ptr;

pub struct PropertiesWithLifespan<'a> {
    pub(crate) properties: Vec<MinifiProperty>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> PropertiesWithLifespan<'a> {
    pub fn new(properties: Vec<MinifiProperty>) -> Self {
        Self {
            properties,
            _marker: std::marker::PhantomData,
        }
    }
}

impl Property {
    pub(crate) fn create_c_default_value_holder(properties: &[Self]) -> Vec<MinifiStringView> {
        properties
            .iter()
            .map(|p| match p.default_value {
                Some(dv) => dv.as_minifi_c_type(),
                None => MinifiStringView {
                    data: ptr::null(),
                    length: 0,
                },
            })
            .collect()
    }

    pub(crate) fn create_c_validators_vec(properties: &[Self]) -> Vec<MinifiPropertyValidator> {
        properties
            .iter()
            .map(|p| unsafe { MinifiGetStandardValidator(p.validator.as_minifi_c_type()) })
            .collect()
    }

    pub(crate) fn create_c_allowed_values_vec_vec(
        properties: &[Self],
    ) -> Vec<Vec<MinifiStringView>> {
        properties
            .iter()
            .map(|p| {
                p.allowed_values
                    .iter()
                    .map(|av| av.as_minifi_c_type())
                    .collect()
            })
            .collect()
    }

    pub(crate) fn create_c_allowed_types_vec_vec(
        properties: &[Self],
    ) -> Vec<Vec<MinifiStringView>> {
        properties
            .iter()
            .map(|p| {
                p.allowed_types
                    .iter()
                    .map(|at| at.as_minifi_c_type())
                    .collect()
            })
            .collect()
    }

    pub(crate) fn create_c_properties<'a>(
        properties: &'a [Self],
        properties_def_value: &'a [MinifiStringView],
        props_allowed_values: &'a [Vec<MinifiStringView>],
        props_allowed_types: &'a [Vec<MinifiStringView>],
        props_validators: &'a [MinifiPropertyValidator],
    ) -> PropertiesWithLifespan<'a> {
        assert_eq!(properties_def_value.len(), properties.len());
        assert_eq!(props_allowed_values.len(), properties.len());
        assert_eq!(props_allowed_types.len(), properties.len());
        assert_eq!(props_validators.len(), properties.len());

        let c_properties = properties
            .iter()
            .zip(properties_def_value.iter())
            .zip(props_allowed_values.iter())
            .zip(props_allowed_types.iter())
            .zip(props_validators.iter())
            .map(
                |((((property, def_value), allowed_values), allowed_types), validator)| {
                    MinifiProperty {
                        name: property.name.as_minifi_c_type(),
                        display_name: property.name.as_minifi_c_type(),
                        description: property.description.as_minifi_c_type(),
                        is_required: property.is_required.as_minifi_c_type(),
                        is_sensitive: property.is_sensitive.as_minifi_c_type(),
                        dependent_properties_count: 0,
                        dependent_properties_ptr: ptr::null(), // Not supported yet.
                        exclusive_of_properties_count: 0,
                        exclusive_of_property_names_ptr: ptr::null(), // Not supported yet.
                        exclusive_of_property_values_ptr: ptr::null(), // Not supported yet.
                        default_value: def_value,
                        allowed_values_count: allowed_values.len() as u32,
                        allowed_values_ptr: allowed_values.as_ptr(),
                        validator: *validator,
                        types_count: allowed_types.len() as u32,
                        types_ptr: allowed_types.as_ptr(),
                        supports_expression_language: property
                            .supports_expr_lang
                            .as_minifi_c_type(),
                    }
                },
            )
            .collect();
        PropertiesWithLifespan::new(c_properties)
    }
}

impl StandardPropertyValidator {
    pub(crate) fn as_minifi_c_type(&self) -> MinifiStandardPropertyValidator {
        match self {
            StandardPropertyValidator::AlwaysValidValidator => {
                MinifiStandardPropertyValidator_MINIFI_ALWAYS_VALID_VALIDATOR
            }
            StandardPropertyValidator::NonBlankValidator => {
                MinifiStandardPropertyValidator_MINIFI_NON_BLANK_VALIDATOR
            }
            StandardPropertyValidator::TimePeriodValidator => {
                MinifiStandardPropertyValidator_MINIFI_TIME_PERIOD_VALIDATOR
            }
            StandardPropertyValidator::BoolValidator => {
                MinifiStandardPropertyValidator_MINIFI_BOOLEAN_VALIDATOR
            }
            StandardPropertyValidator::I64Validator => {
                MinifiStandardPropertyValidator_MINIFI_INTEGER_VALIDATOR
            }
            StandardPropertyValidator::U64Validator => {
                MinifiStandardPropertyValidator_MINIFI_UNSIGNED_INTEGER_VALIDATOR
            }
            StandardPropertyValidator::DataSizeValidator => {
                MinifiStandardPropertyValidator_MINIFI_DATA_SIZE_VALIDATOR
            }
            StandardPropertyValidator::PortValidator => {
                MinifiStandardPropertyValidator_MINIFI_PORT_VALIDATOR
            }
        }
    }
}
