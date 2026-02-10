use super::PGPPrivateKeyService;
use super::properties::*;
use minifi_native::{
    ControllerServiceDefinition, RegisterableControllerService,
};

impl RegisterableControllerService for PGPPrivateKeyService {
    fn get_definition() -> Box<dyn minifi_native::DynControllerServiceDefinition> {
        Box::new(ControllerServiceDefinition::<PGPPrivateKeyService>::new(
            "PGP Public Key Service providing Public Keys loaded from files",
            &[KEY_FILE, KEY, KEY_PASSPHRASE],
        ))
    }
}
