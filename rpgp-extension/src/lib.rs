mod processors;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!([
    processors::encrypt_content_pgp::EncryptContentPGP,
]);
