use super::PGPPublicKeyService;
use super::properties::*;
use minifi_native::{ControllerServiceDefinition, Property};

impl ControllerServiceDefinition for PGPPublicKeyService {
    const DESCRIPTION: &'static str =
        "PGP Public Key Service providing Public Keys loaded from files";
    const PROPERTIES: &'static [Property] = &[KEYRING_FILE, KEYRING];
}
