mod controller_services;
mod processors;

#[cfg(not(test))]
use crate::controller_services::private_key_service::PGPPrivateKeyService;
#[cfg(not(test))]
use crate::controller_services::public_key_service::PGPPublicKeyService;
#[cfg(not(test))]
use crate::processors::decrypt_content::DecryptContentPGP;
#[cfg(not(test))]
use crate::processors::encrypt_content::EncryptContentPGP;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(
    processors: [
        minifi_native::TransformFlowFileProcessor::<EncryptContentPGP>,
        minifi_native::TransformFlowFileProcessor::<DecryptContentPGP>,
    ],
    controllers: [
        minifi_native::ControllerService::<PGPPublicKeyService>,
        minifi_native::ControllerService::<PGPPrivateKeyService>,
    ]
);

#[cfg(test)]
mod test_utils;
