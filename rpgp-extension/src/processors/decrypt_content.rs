mod properties;
mod relationships;

use crate::controller_services::private_key_service::PrivateKeyService;
use crate::processors::decrypt_content::properties::{PRIVATE_KEY_SERVICE, SYMMETRIC_PASSWORD};
use crate::processors::decrypt_content::relationships::{FAILURE, SUCCESS};
use minifi_native::{ConstTriggerable, Logger, MetricsProvider, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, Schedulable};
use pgp::composed::TheRing;
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
enum DecryptionStrategy {
    Decrypted,
    Packaged,
}

#[derive(Debug)]
pub(crate) struct DecryptContent {
    decompress_data: bool,
    symmetric_password: Option<pgp::types::Password>,
}

impl Schedulable for DecryptContent {
    fn schedule<P: ProcessContext, L>(
        context: &P,
        _logger: &L,
    ) -> Result<Self, MinifiError>
    where
        Self: Sized,
        L: Logger,
    {
        let decryption_strategy = context
            .get_property(&properties::DECRYPTION_STRATEGY, None)?
            .expect("required property")
            .parse::<DecryptionStrategy>()?;

        let symmetric_password = context
            .get_property(&SYMMETRIC_PASSWORD, None)?
            .and_then(|pwd_str| Option::from(pgp::types::Password::from(pwd_str)));
        let has_context_service = context.get_property(&PRIVATE_KEY_SERVICE, None)?.is_some();
        if !has_context_service && symmetric_password.is_none() {
            Err(MinifiError::ScheduleError(
                "Either Symmetric Password or Private Key Service must be set".to_string(),
            ))
        } else {
            Ok(DecryptContent {
                decompress_data: decryption_strategy == DecryptionStrategy::Decrypted,
                symmetric_password,
            })
        }
    }
}

impl DecryptContent {
    fn decrypt_msg<'a, PC, L>(
        &self,
        msg: pgp::composed::Message<'a>,
        ctx: &PC,
        _logger: &L,
    ) -> pgp::errors::Result<pgp::composed::Message<'a>>
    where
        PC: ProcessContext,
        L: Logger,
    {
        let private_key_service = ctx
            .get_controller_service::<PrivateKeyService>(&PRIVATE_KEY_SERVICE)
            .unwrap_or(None);
        let mut ring = if let Some(pks) = private_key_service {
            pks.get_the_ring()
        } else {
            TheRing::default()
        };

        ring.decrypt_options = ring.decrypt_options.enable_gnupg_aead();

        if let Some(sym_passwd) = &self.symmetric_password {
            ring.message_password.push(sym_passwd);
        }
        let (decrypted_msg, _ring_result) = msg.decrypt_the_ring(ring, false)?;
        Ok(decrypted_msg)
    }

    fn get_msg<PS>(
        &'_ self,
        session: &mut PS,
        ff: &PS::FlowFile,
    ) -> Option<pgp::composed::Message<'_>>
    where
        PS: ProcessSession,
    {
        session
            .read(&ff)
            .and_then(|bytes| {
                pgp::composed::Message::from_reader(std::io::Cursor::new(bytes)).map(|(msg, _header)| {
                    msg
                }).ok()
            })
    }
}

impl ConstTriggerable for DecryptContent {
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
        let mut ff = session.get();
        if ff.is_none() {
            return Ok(OnTriggerResult::Yield);
        }

        if let Some(mut decrypted_msg) = self
            .get_msg(session, ff.as_ref().unwrap())
            .and_then(|msg| self.decrypt_msg(msg, context, logger).map_err(|e| logger.trace(&format!("Couldnt decrypt message due to {:?}", e))).ok())
        {

            if self.decompress_data && decrypted_msg.is_compressed() {
                match decrypted_msg.decompress() {
                    Ok(decompressed_data) => {
                        decrypted_msg = decompressed_data;
                    }
                    Err(e) => {
                        logger.warn(&format!("Failed to decompress message {:?}", e));
                        session.transfer(ff.unwrap(), &FAILURE.name);
                        return Ok(OnTriggerResult::Ok);
                    }
                }
            }
            if let Ok(data_vec) = decrypted_msg.as_data_vec() {
                session.write(&mut ff.as_mut().unwrap(), &data_vec);
                session.transfer(ff.unwrap(), &SUCCESS.name);
                Ok(OnTriggerResult::Ok)
            } else {
                logger.warn("Failed to serialize decrypted message");
                session.transfer(ff.unwrap(), &FAILURE.name);
                Ok(OnTriggerResult::Ok)
            }
        } else {
            session.transfer(ff.unwrap(), &FAILURE.name);
            Ok(OnTriggerResult::Ok)
        }
    }
}

impl MetricsProvider for DecryptContent {}

#[cfg(test)]
mod tests;

#[cfg(not(test))]
mod processor_definition;
