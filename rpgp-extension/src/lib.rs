mod processors;
mod controller_services;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!([
    processors::encrypt_content_pgp::EncryptContentPGP,
]);
