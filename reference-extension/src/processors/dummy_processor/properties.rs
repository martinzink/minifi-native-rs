use minifi_native::{Property, StandardPropertyValidator};

pub(crate) const CONTROLLER_SERVICE: Property = Property {
    name: "Dummy Controller Service",
    description: "Name of the dummy controller service",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "rs::DummyControllerService", // TODO(mzink) cannot call non-const associated function in constants
};
