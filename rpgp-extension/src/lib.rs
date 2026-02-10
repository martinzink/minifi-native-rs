mod controller_services;
mod processors;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(
    processors: [
        minifi_native::FlowFileTransformer::<processors::encrypt_content::EncryptContent>,
        minifi_native::FlowFileTransformer::<processors::decrypt_content::DecryptContent>,
    ],
    controllers: [
        controller_services::public_key_service::PublicKeyService,
        controller_services::private_key_service::PrivateKeyService,
    ]
);

#[cfg(test)]
mod test_utils;
