mod controller_service_definition;
mod properties;

use crate::controller_services::pgp_public_key_service::properties::{KEYRING, KEYRING_FILE};
use minifi_native::{
    ControllerService, ControllerServiceContext, DefaultLogger, LogLevel, Logger, MinifiError,
};
use pgp::composed::{Deserializable, SignedPublicKey};

#[derive(Debug)]
pub(crate) struct PgpPublicKeyService {
    logger: DefaultLogger,
    public_keys: Vec<SignedPublicKey>,
}

impl ControllerService for PgpPublicKeyService {
    fn new(logger: DefaultLogger) -> Self {
        PgpPublicKeyService {
            logger,
            public_keys: Vec::new(),
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn enable<P: ControllerServiceContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        if let Some(keyring_file_path) = context.get_property(&KEYRING_FILE)? {
            if let Ok((keys, _headers)) = SignedPublicKey::from_armor_file_many(&keyring_file_path)
            {
                self.public_keys.extend(keys.filter_map(|key| key.ok()));
            } else if let Ok(keys) = SignedPublicKey::from_file_many(keyring_file_path) {
                self.public_keys.extend(keys.filter_map(|key| key.ok()));
            }
        }
        if let Some(keyring_ascii) = context.get_property(&KEYRING)? {
            if let Ok((keys, _headers)) = SignedPublicKey::from_armor_many(keyring_ascii.as_bytes())
            {
                self.public_keys.extend(keys.filter_map(|key| key.ok()));
            }
        }

        if self.public_keys.is_empty() {
            return Err(MinifiError::ControllerServiceError(
                "Could not load any valid keys",
            ));
        }
        Ok(())
    }

    fn class_name() -> &'static str {
        "PgpPublicKeyService" // TODO(mzink)
    }

    fn group_name() -> &'static str {
        env!("CARGO_PKG_NAME") // TODO(mzink)
    }
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION") // TODO(mzink)
    }
}

impl PgpPublicKeyService {
    pub fn get(&self, target_id: &str) -> Option<&SignedPublicKey> {
        self.public_keys.iter().find(|public_key| {
            public_key.details.users.iter().any(|user| {
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
