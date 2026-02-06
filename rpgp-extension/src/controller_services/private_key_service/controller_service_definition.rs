use super::{PrivateKeyService};
use super::properties::*;
use minifi_native::{
    ControllerService, ControllerServiceDefinition, RegisterableControllerService,
};

impl RegisterableControllerService for PrivateKeyService {
    fn get_definition() -> Box<dyn minifi_native::DynControllerServiceDefinition> {
        Box::new(ControllerServiceDefinition::<PrivateKeyService>::new(
            PrivateKeyService::class_name(),
            "PGP Public Key Service providing Public Keys loaded from files",
            &[KEY_FILE, KEY, KEY_PASSPHRASE],
        ))
    }
}
