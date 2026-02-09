use minifi_native::{
    ConstTriggerable, Logger, MetricsProvider, MinifiError, OnTriggerResult, ProcessContext,
    ProcessSession, Schedulable,
};
use pgp::composed::{ArmorOptions, MessageBuilder, SignedPublicKey};
use pgp::types::{StringToKey};

mod properties;
mod relationships;
mod output_attributes;


use crate::controller_services::public_key_service::PublicKeyService;
use crate::processors::encrypt_content::properties::{
    PASSPHRASE, PUBLIC_KEY_SEARCH, PUBLIC_KEY_SERVICE,
};
use crate::processors::encrypt_content::relationships::{FAILURE, SUCCESS};
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};
use crate::processors::encrypt_content::output_attributes::FILE_ENCODING;

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE", const_into_str)]
enum FileEncoding {
    Ascii,
    Binary,
}

#[derive(Debug)]
pub(crate) struct EncryptContentPGP {
    file_encoding: FileEncoding,
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
            builder.encrypt_with_password(
                StringToKey::new_argon2(rand::thread_rng(), 3, 4, 16),
                &passphrase.into(),
            )?;
        }

        match self.file_encoding {
            FileEncoding::Ascii => builder
                .to_armored_string(rand::thread_rng(), ArmorOptions::default())
                .map(|s| s.as_bytes().to_vec()),
            FileEncoding::Binary => builder.to_vec(rand::thread_rng()),
        }
    }
}

impl Schedulable for EncryptContentPGP {
    fn schedule<P: ProcessContext, L: Logger>(context: &P, _logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
        let file_encoding = context
            .get_property(&properties::FILE_ENCODING, None)?
            .expect("required property")
            .parse::<FileEncoding>()?;

        Ok(EncryptContentPGP { file_encoding })
    }
}

impl ConstTriggerable for EncryptContentPGP {
    fn trigger<PC, PS, L>(
        &self,
        context: &mut PC,
        session: &mut PS,
        logger: &L,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
        L: Logger,
    {
        if let Some(mut flow_file) = session.get() {
            let public_key = if let (Some(pub_key_search), Some(public_key_service)) = (
                context.get_property(&PUBLIC_KEY_SEARCH, Some(&flow_file))?,
                context.get_controller_service::<PublicKeyService>(&PUBLIC_KEY_SERVICE)?,
            ) {
                public_key_service.get(&pub_key_search)
            } else {
                None
            };

            let password = context.get_property(&PASSPHRASE, Some(&flow_file))?;
            if public_key.is_none() && password.is_none() {
                logger.warn("The was no public key or password provided");
                session.transfer(flow_file, FAILURE.name);
            } else if let Some(content) = session.read(&flow_file) {
                if let Ok(encrypted_content) =
                    self.encrypt_message(content, public_key, password.as_deref())
                {
                    session.write(&mut flow_file, &encrypted_content);
                    session.set_attribute(&mut flow_file, FILE_ENCODING.name, &self.file_encoding.to_string());
                    session.transfer(flow_file, SUCCESS.name);
                } else {
                    session.transfer(flow_file, FAILURE.name);
                }
            }

            Ok(OnTriggerResult::Ok)
        } else {
            Ok(OnTriggerResult::Yield)
        }
    }
}

impl MetricsProvider for EncryptContentPGP {}

#[cfg(test)]
mod tests;

#[cfg(not(test))]
mod processor_definition;
