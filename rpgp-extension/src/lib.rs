mod controller_services;
mod processors;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(
    processors: [
        minifi_native::FlowFileTransformer::<processors::encrypt_content::EncryptContentPGP>,
        minifi_native::FlowFileTransformer::<processors::decrypt_content::DecryptContentPGP>,
    ],
    controllers: [
        controller_services::public_key_service::PGPPublicKeyService,
        controller_services::private_key_service::PGPPrivateKeyService,
    ]
);

#[cfg(test)]
mod test_utils;
