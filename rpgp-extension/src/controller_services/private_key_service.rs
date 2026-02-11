mod controller_service_definition;
mod properties;

use minifi_native::{
    ControllerService, ControllerServiceContext, DefaultLogger, IdentifyComponent, LogLevel,
    Logger, MinifiError,
};
use pgp::composed::{Deserializable, SignedSecretKey, TheRing};
use pgp::types::Password;

#[derive(Debug, IdentifyComponent)]
pub(crate) struct PGPPrivateKeyService {
    logger: DefaultLogger,
    private_keys: Vec<SignedSecretKey>,
    passphrase: Password,
}

impl ControllerService for PGPPrivateKeyService {
    fn new(logger: DefaultLogger) -> Self {
        PGPPrivateKeyService {
            logger,
            private_keys: Vec::new(),
            passphrase: Password::empty(),
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        if let Some(keyring_file_path) = context.get_property(&properties::KEY_FILE)? {
            if let Ok((keys, _headers)) = SignedSecretKey::from_armor_file_many(&keyring_file_path)
            {
                self.private_keys.extend(keys.filter_map(|key| key.ok()));
            } else if let Ok(keys) = SignedSecretKey::from_file_many(keyring_file_path) {
                self.private_keys.extend(keys.filter_map(|key| key.ok()));
            }
        }
        if let Some(keyring_ascii) = context.get_property(&properties::KEY)? {
            if let Ok((keys, _headers)) = SignedSecretKey::from_armor_many(keyring_ascii.as_bytes())
            {
                self.private_keys.extend(keys.filter_map(|key| key.ok()));
            }
        }

        if let Some(passphrase_str) = context.get_property(&properties::KEY_PASSPHRASE)? {
            self.passphrase = Password::from(passphrase_str);
        }

        if self.private_keys.is_empty() {
            return Err(MinifiError::ControllerServiceError(
                "Could not load any valid keys",
            ));
        }
        Ok(())
    }
}

impl PGPPrivateKeyService {
    pub fn get_the_ring(&'_ self) -> TheRing<'_> {
        TheRing {
            secret_keys: self.private_keys.iter().collect(),
            key_passwords: vec![&self.passphrase],
            message_password: vec![],
            session_keys: vec![],
            decrypt_options: Default::default(),
        }
    }

    #[cfg(test)]
    pub fn get_secret_key(&self, target_id: &str) -> Option<&SignedSecretKey> {
        self.private_keys.iter().find(|private_key| {
            private_key.details.users.iter().any(|user| {
                user.id
                    .as_str()
                    .map(|user_id| user_id.contains(target_id))
                    .unwrap_or(false)
            })
        })
    }
}

#[cfg(test)]
mod tests;
