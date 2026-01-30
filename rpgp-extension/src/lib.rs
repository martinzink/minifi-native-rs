mod processors;
mod controller_services;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(processors: [
        minifi_native::MultiThreadedProcessor::<processors::encrypt_content_pgp::EncryptContentPGP>,
], controllers: [
    controller_services::pgp_public_key_service::PgpPublicKeyService
]);

#[cfg(test)]
mod test_utils;