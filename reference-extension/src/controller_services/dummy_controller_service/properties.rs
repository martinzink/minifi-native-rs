use minifi_native::{Property, StandardPropertyValidator};

pub(crate) const DATA: Property = Property {
    name: "Data",
    description: "data",
    is_required: true,
    is_sensitive: true,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};
