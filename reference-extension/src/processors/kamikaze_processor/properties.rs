use crate::processors::kamikaze_processor::KamikazeBehaviour;
use minifi_native::{Property, StandardPropertyValidator};
use strum::VariantNames;

pub(crate) const ON_SCHEDULE_BEHAVIOUR: Property = Property {
    name: "On Schedule Behaviour",
    description: "What to do during the on_schedule method",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("ReturnOk"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &KamikazeBehaviour::VARIANTS,
    allowed_types: &[],
};

pub(crate) const ON_TRIGGER_BEHAVIOUR: Property = Property {
    name: "On Trigger Behaviour",
    description: "What to do during the on_trigger method",
    is_required: true,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: Some("ReturnOk"),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &KamikazeBehaviour::VARIANTS,
    allowed_types: &[],
};

pub(crate) const READ_BEHAVIOUR: Property = Property {
    name: "Read Behaviour",
    description: "If specified it will process incoming flowfiles with the specified behaviour.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &KamikazeBehaviour::VARIANTS,
    allowed_types: &[],
};

pub(crate) const WRITE_BEHAVIOUR: Property = Property {
    name: "Write Behaviour",
    description: "If specified it will create and process new flowfiles with the specified behaviour.",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &KamikazeBehaviour::VARIANTS,
    allowed_types: &[],
};
