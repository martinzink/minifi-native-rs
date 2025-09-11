use minifi_native::{Property, StandardPropertyValidator};

pub(crate) const CONTENT: Property = Property {
    name: "Content",
    description: "What to write to the flowfile.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: true,
    default_value: Some("Something default to write"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_types: &[],
};

pub(crate) const SHOUT: Property = Property {
    name: "Shouting",
    description: "do you want to shout?",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("false"),
    validator: StandardPropertyValidator::BoolValidator,
    allowed_values: &[],
    allowed_types: &[],
};
