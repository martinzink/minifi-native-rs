use minifi_native::{FlowFileTransform, InputStream, Logger, MinifiError, ProcessContext, Schedule, TransformedFlowFile};
use pgp::composed::{ArmorOptions, MessageBuilder, SignedPublicKey};
use pgp::types::StringToKey;
use std::collections::HashMap;

mod output_attributes;
mod properties;
mod relationships;

use crate::controller_services::public_key_service::PGPPublicKeyService;
use crate::processors::encrypt_content::output_attributes::FILE_ENCODING;
use crate::processors::encrypt_content::properties::{
    PASSPHRASE, PUBLIC_KEY_SEARCH, PUBLIC_KEY_SERVICE,
};
use crate::processors::encrypt_content::relationships::{FAILURE, SUCCESS};
use minifi_native::macros::{ComponentIdentifier, DefaultMetrics};
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE", const_into_str)]
enum FileEncoding {
    Ascii,
    Binary,
}

#[derive(Debug, ComponentIdentifier, DefaultMetrics)]
pub(crate) struct EncryptContentPGP {
    file_encoding: FileEncoding,
}

#[cfg(not(test))]
fn string_to_key() -> StringToKey {
    StringToKey::new_argon2(rand::thread_rng(), 3, 4, 16) // 64 MiB with rpgp's recommended parameter choice
}

#[cfg(test)]
fn string_to_key() -> StringToKey {
    StringToKey::new_argon2(rand::thread_rng(), 1, 1, 10) // fast for unit tests
}

impl EncryptContentPGP {
    fn encrypt_message(
        &self,
        message: Vec<u8>,
        pub_key: Option<&SignedPublicKey>,
        passphrase: Option<&str>,
    ) -> pgp::errors::Result<Vec<u8>> {
        let mut builder = MessageBuilder::from_bytes("", message).seipd_v1(
            rand::thread_rng(),
            pgp::crypto::sym::SymmetricKeyAlgorithm::AES256,
        );

        if let Some(pub_key) = pub_key {
            builder.encrypt_to_key(rand::thread_rng(), &pub_key)?;
        }

        if let Some(passphrase) = passphrase {
            builder.encrypt_with_password(string_to_key(), &passphrase.into())?;
        }

        match self.file_encoding {
            FileEncoding::Ascii => builder
                .to_armored_string(rand::thread_rng(), ArmorOptions::default())
                .map(|s| s.as_bytes().to_vec()),
            FileEncoding::Binary => builder.to_vec(rand::thread_rng()),
        }
    }
}

impl Schedule for EncryptContentPGP {
    fn schedule<P: ProcessContext, L: Logger>(context: &P, _logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        let file_encoding = context
            .get_property(&properties::FILE_ENCODING, None)?
            .expect("required property")
            .parse::<FileEncoding>()?;

        let has_password = context.get_property(&PASSPHRASE, None)?.is_some();
        let has_public_key = context.get_property(&PUBLIC_KEY_SERVICE, None)?.is_some()
            && context.get_property(&PUBLIC_KEY_SEARCH, None)?.is_some();

        if !has_password && !has_public_key {
            Err(MinifiError::ScheduleError(
                "Either a password or Public Key Service with Public Key Search should be configured to encrypt files"
                    .to_string(),
            ))
        } else {
            Ok(EncryptContentPGP { file_encoding })
        }
    }
}

impl FlowFileTransform for EncryptContentPGP {
    fn transform<
        'a,
        Context: ProcessContext,
        LoggerImpl: Logger,
    >(
        &self,
        context: &'a mut Context,
        flow_file: &Context::FlowFile,
        input_stream: &'a mut dyn InputStream,
        logger: &LoggerImpl,
    ) -> Result<TransformedFlowFile<'a>, MinifiError> {
        let public_key = if let (Some(pub_key_search), Some(public_key_service)) = (
            context.get_property(&PUBLIC_KEY_SEARCH, Some(&flow_file))?,
            context.get_controller_service::<PGPPublicKeyService>(&PUBLIC_KEY_SERVICE)?,
        ) {
            public_key_service.get(&pub_key_search)
        } else {
            None
        };
        let password = context.get_property(&PASSPHRASE, Some(&flow_file))?;
        if public_key.is_none() && password.is_none() {
            logger.debug("No password or public key to encrypt with");
            return Ok(TransformedFlowFile::route_without_changes(
                &FAILURE,
            ));
        }

        let mut content = Vec::new();
        let _content_size = input_stream.read_to_end(&mut content);

        match self.encrypt_message(content, public_key.as_deref(), password.as_deref()) {
            Ok(encrypted_content) => Ok(TransformedFlowFile::new(
                &SUCCESS,
                Some(encrypted_content),
                HashMap::from([(
                    FILE_ENCODING.name.to_string(),
                    self.file_encoding.to_string(),
                )]),
            )),
            Err(e) => {
                logger.debug(&format!("Failed to encrypt content {:?}", e));
                Ok(TransformedFlowFile::route_without_changes(
                    &FAILURE,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(not(test))]
mod processor_definition;
