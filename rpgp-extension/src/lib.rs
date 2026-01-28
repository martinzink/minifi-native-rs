mod processors;
mod controller_services;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(processors: [
    processors::encrypt_content_pgp::EncryptContentPGP,
], controllers: [
    controller_services::pgp_public_key_service::PgpPublicKeyService
]);
