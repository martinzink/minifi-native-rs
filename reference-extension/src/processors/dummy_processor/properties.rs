use minifi_native::ComponentIdentifier;
use minifi_native::{Property, StandardPropertyValidator};
use crate::controller_services::dummy_controller_service::DummyControllerService;

pub(crate) const CONTROLLER_SERVICE: Property = Property {
    name: "Dummy Controller Service",
    description: "Name of the dummy controller service",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: DummyControllerService::CLASS_NAME
};
