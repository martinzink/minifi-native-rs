use super::{PgpPublicKeyService};
use super::properties::*;
use minifi_native::{
    ControllerService, ControllerServiceDefinition, RegisterableControllerService,
};

impl RegisterableControllerService for PgpPublicKeyService {
    fn get_definition() -> Box<dyn minifi_native::DynControllerServiceDefinition> {
        Box::new(ControllerServiceDefinition::<PgpPublicKeyService>::new(
            PgpPublicKeyService::class_name(),
            "PGP Public Key Service providing Public Keys loaded from files",
            &[KEYRING_FILE, KEYRING],
        ))
    }
}
