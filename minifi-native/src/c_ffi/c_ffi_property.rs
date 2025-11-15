use super::c_ffi_primitives::{StaticStrAsMinifiCStr};
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

#[allow(dead_code)] // the c_ vecs are holding the values referenced from the properties
pub struct CProperties {
    c_default_values: Vec<MinifiStringView>,
    c_allowed_values: Vec<Vec<MinifiStringView>>,
    c_allowed_types: Vec<MinifiStringView>,
    c_validators: Vec<*const MinifiPropertyValidator>,
    properties: Vec<MinifiProperty>,
}

impl CProperties {
    pub(crate) fn new(
        c_default_values: Vec<MinifiStringView>,
        c_allowed_values: Vec<Vec<MinifiStringView>>,
        c_allowed_types: Vec<MinifiStringView>,
        c_validators: Vec<*const MinifiPropertyValidator>,
        properties: Vec<MinifiProperty>,
    ) -> Self {
        Self {
            c_default_values,
            c_allowed_values,
            c_allowed_types,
            c_validators,
            properties,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.properties.len()
    }

    pub(crate) unsafe fn get_ptr(&self) -> *const MinifiProperty {
        self.properties.as_ptr()
    }
}

impl Property {
    fn create_c_default_value_holder(properties: &[Self]) -> Vec<MinifiStringView> {
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

    fn create_c_validators_vec(properties: &[Self]) -> Vec<*const MinifiPropertyValidator> {
        properties
            .iter()
            .map(|p| unsafe { MinifiGetStandardValidator(p.validator.as_minifi_c_type()) })
            .collect()
    }

    fn create_c_allowed_values_vec_vec(properties: &[Self]) -> Vec<Vec<MinifiStringView>> {
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

    fn create_c_allowed_types_vec(properties: &[Self]) -> Vec<MinifiStringView> {
        properties
            .iter()
            .map(|p| p.allowed_type.as_minifi_c_type())
            .collect()
    }

    pub(crate) fn create_c_properties(properties: &[Self]) -> CProperties {
        let c_default_values = Property::create_c_default_value_holder(properties);
        let c_allowed_values = Property::create_c_allowed_values_vec_vec(properties);
        let c_allowed_types = Property::create_c_allowed_types_vec(properties);
        let c_validators = Property::create_c_validators_vec(properties);
        assert_eq!(c_default_values.len(), properties.len());
        assert_eq!(c_allowed_values.len(), properties.len());
        assert_eq!(c_allowed_types.len(), properties.len());
        assert_eq!(c_validators.len(), properties.len());

        let c_properties = properties
            .iter()
            .zip(c_default_values.iter())
            .zip(c_allowed_values.iter())
            .zip(c_allowed_types.iter())
            .zip(c_validators.iter())
            .map(
                |((((property, def_value), allowed_values), allowed_type), validator)| {
                    MinifiProperty {
                        name: property.name.as_minifi_c_type(),
                        display_name: property.name.as_minifi_c_type(),
                        description: property.description.as_minifi_c_type(),
                        is_required: property.is_required,
                        is_sensitive: property.is_sensitive,
                        default_value: def_value,
                        allowed_values_count: allowed_values.len(),
                        allowed_values_ptr: allowed_values.as_ptr(),
                        validator: *validator,
                        type_: allowed_type,
                        supports_expression_language: property
                            .supports_expr_lang,
                    }
                },
            )
            .collect();
        CProperties::new(
            c_default_values,
            c_allowed_values,
            c_allowed_types,
            c_validators,
            c_properties,
        )
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
