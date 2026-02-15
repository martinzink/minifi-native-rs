mod controller_services;
mod processors;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(
    processors: [
        minifi_native::FlowFileTransformer::<processors::encrypt_content::EncryptContentPGP>,
        minifi_native::FlowFileTransformer::<processors::decrypt_content::DecryptContentPGP>,
    ],
    controllers: [
        minifi_native::ControllerService::<controller_services::public_key_service::PGPPublicKeyService>,
        minifi_native::ControllerService::<controller_services::private_key_service::PGPPrivateKeyService>,
    ]
);

#[cfg(test)]
mod test_utils;
