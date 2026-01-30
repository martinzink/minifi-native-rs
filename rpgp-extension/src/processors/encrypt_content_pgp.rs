use minifi_native::{
    Concurrent, ConcurrentOnTrigger, DefaultLogger, LogLevel, Logger, MinifiError, OnTriggerResult,
    ProcessContext, ProcessSession, RawProcessor,
};
use pgp::composed::{ArmorOptions, MessageBuilder, SignedPublicKey};
use pgp::types::StringToKey;

mod processor_definition;
mod properties;
mod relationships;

use crate::controller_services::pgp_public_key_service::PgpPublicKeyService;
use crate::processors::encrypt_content_pgp::properties::{
    PASSPHRASE, PUBLIC_KEY_SEARCH, PUBLIC_KEY_SERVICE,
};
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};
use crate::processors::encrypt_content_pgp::relationships::{FAILURE, SUCCESS};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
enum FileEncoding {
    Ascii,
    Binary,
}

#[derive(Debug)]
struct ScheduledMembers {
    file_encoding: FileEncoding,
}

impl ScheduledMembers {
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
            FileEncoding::Ascii => {
                builder.to_armored_string(rand::thread_rng(), ArmorOptions::default()).map(|s| s.as_bytes().to_vec())
            }
            FileEncoding::Binary => {
                builder.to_vec(rand::thread_rng())
            }
        }
    }

    fn on_trigger<P: ProcessContext, S: ProcessSession<FlowFile = P::FlowFile>>(
        &self,
        context: &mut P,
        session: &mut S,
    ) -> Result<OnTriggerResult, MinifiError> {
        if let Some(mut flow_file) = session.get() {
            let public_key = if let (Some(pub_key_search), Some(public_key_service)) = (
                context.get_property(&PUBLIC_KEY_SEARCH, Some(&flow_file))?,
                context.get_controller_service::<PgpPublicKeyService>(&PUBLIC_KEY_SERVICE)?,
            ) {
                public_key_service.get(&pub_key_search)
            } else {
                None
            };

            let password = context.get_property(&PASSPHRASE, Some(&flow_file))?;
            if public_key.is_none() && password.is_none() {
                session.transfer(flow_file, FAILURE.name);
            } else if let Some(content) = session.read(&flow_file) {
                if let Ok(encrypted_content) = self.encrypt_message(content, public_key, password.as_deref()) {
                    session.write(&mut flow_file, &encrypted_content);
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

#[derive(Debug)]
pub(crate) struct EncryptContentPGP {
    logger: DefaultLogger,
    scheduled_members: Option<ScheduledMembers>,
}

impl RawProcessor for EncryptContentPGP {
    type Threading = Concurrent;

    fn new(logger: DefaultLogger) -> Self {
        Self {
            logger,
            scheduled_members: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        let file_encoding = context
            .get_property(&properties::FILE_ENCODING, None)?
            .expect("required property")
            .parse::<FileEncoding>()?;

        self.scheduled_members = Some(ScheduledMembers {
            file_encoding,
        });

        Ok(())
    }
}

impl ConcurrentOnTrigger for EncryptContentPGP {
    fn on_trigger<PC, PS>(
        &self,
        context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
    {
        if let Some(ref s) = self.scheduled_members {
            s.on_trigger(context, session)
        } else {
            Err(MinifiError::TriggerError(
                "The processor hasn't been scheduled yet".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests;
