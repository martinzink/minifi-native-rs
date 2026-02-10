use super::PGPPublicKeyService;
use super::properties::*;
use minifi_native::{
    ControllerServiceDefinition, RegisterableControllerService,
};

impl RegisterableControllerService for PGPPublicKeyService {
    fn get_definition() -> Box<dyn minifi_native::DynControllerServiceDefinition> {
        Box::new(ControllerServiceDefinition::<PGPPublicKeyService>::new(
            "PGP Public Key Service providing Public Keys loaded from files",
            &[KEYRING_FILE, KEYRING],
        ))
    }
}
