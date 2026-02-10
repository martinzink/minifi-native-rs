use minifi_native::ComponentIdentifier;
use minifi_native::{Property, StandardPropertyValidator};
use strum::VariantNames;
use crate::controller_services::private_key_service::PGPPrivateKeyService;

pub(crate) const DECRYPTION_STRATEGY: Property = Property {
    name: "Decryption Strategy",
    description: "Strategy for writing files to success after decryption",
    is_required: false,
    is_sensitive: true,
    supports_expr_lang: false,
    default_value: Some(super::DecryptionStrategy::Decrypted.into_str()),
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &super::DecryptionStrategy::VARIANTS,
    allowed_type: "",
};

pub(crate) const SYMMETRIC_PASSWORD: Property = Property {
    name: "Symmetric Password",
    description: "Password used for decrypting data encrypted with Password-Based Encryption",
    is_required: false,
    is_sensitive: true,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: "",
};

pub(crate) const PRIVATE_KEY_SERVICE: Property = Property {
    name: "Private Key Service",
    description: "PGP Private Key Service for decrypting data encrypted with Public Key Encryption",
    is_required: false,
    is_sensitive: false,
    supports_expr_lang: false,
    default_value: None,
    validator: StandardPropertyValidator::AlwaysValidValidator,
    allowed_values: &[],
    allowed_type: PGPPrivateKeyService::CLASS_NAME,
};
