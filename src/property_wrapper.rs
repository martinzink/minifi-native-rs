use std::ptr;
use crate::primitives::{create_bool, create_string_view};
use minificpp_sys::MinifiProperty;

pub struct Property {
    pub c_struct: MinifiProperty,
}

impl Property {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        is_required: bool,
        is_sensitive: bool,
        supports_expr_lang: bool,
    ) -> Self {
        Self {
            c_struct: MinifiProperty {
                name: create_string_view(name),
                display_name: create_string_view(name),
                description: create_string_view(description),
                is_required: create_bool(is_required),
                is_sensitive: create_bool(is_sensitive),
                dependent_properties_count: 0,
                dependent_properties_ptr: ptr::null(),
                exclusive_of_properties_count: 0,
                exclusive_of_property_names_ptr: ptr::null(),
                exclusive_of_property_values_ptr: ptr::null(),
                default_value: ptr::null(),
                allowed_values_count: 0,
                allowed_values_ptr: ptr::null(),
                validator: ptr::null(),
                types_count: 0,
                types_ptr: ptr::null(),
                supports_expression_language: create_bool(supports_expr_lang),
            },
        }
    }
}
