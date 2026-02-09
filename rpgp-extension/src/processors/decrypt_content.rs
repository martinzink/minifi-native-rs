mod properties;
mod relationships;
mod output_attributes;

use crate::controller_services::private_key_service::PrivateKeyService;
use crate::processors::decrypt_content::properties::{PRIVATE_KEY_SERVICE, SYMMETRIC_PASSWORD};
use crate::processors::decrypt_content::relationships::{FAILURE, SUCCESS};
use minifi_native::{
    ConstTrigger, Logger, CalculateMetrics, MinifiError, OnTriggerResult, ProcessContext,
    ProcessSession, Schedule,
};
use pgp::composed::TheRing;
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE", const_into_str)]
enum DecryptionStrategy {
    Decrypted,
    Packaged,
}

#[derive(Debug)]
pub(crate) struct DecryptContent {
    decompress_data: bool,
    symmetric_password: Option<pgp::types::Password>,
}

impl Schedule for DecryptContent {
    fn schedule<P: ProcessContext, L>(context: &P, _logger: &L) -> Result<Self, MinifiError>
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
        session.read(&ff).and_then(|bytes| {
            pgp::composed::Message::from_reader(std::io::Cursor::new(bytes))
                .map(|(msg, _header)| msg)
                .ok()
        })
    }
}

impl ConstTrigger for DecryptContent {
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
        let mut attributes: Vec<(String, String)> = Vec::new();
        let ff = session.get();
        if ff.is_none() {
            return Ok(OnTriggerResult::Yield);
        }

        let mut ff = ff.unwrap();

        if let Some(mut decrypted_msg) =
            self.get_msg(session, &ff).and_then(|msg| {
                self.decrypt_msg(msg, context, logger)
                    .map_err(|e| logger.trace(&format!("Couldnt decrypt message due to {:?}", e)))
                    .ok()
            })
        {
            if self.decompress_data && decrypted_msg.is_compressed() {
                match decrypted_msg.decompress() {
                    Ok(decompressed_data) => {
                        decrypted_msg = decompressed_data;
                    }
                    Err(e) => {
                        logger.warn(&format!("Failed to decompress message {:?}", e));
                        session.transfer(ff, &FAILURE.name);
                        return Ok(OnTriggerResult::Ok);
                    }
                }
            }
            if let Ok(data_vec) = decrypted_msg.as_data_vec() {
                if let Some(literal_data_header) = decrypted_msg.literal_data_header() {
                    if let Ok(file_name) = str::from_utf8(literal_data_header.file_name()) {
                        attributes.push((output_attributes::LITERAL_DATA_FILENAME.name.to_string(), file_name.to_string()));
                    }
                    attributes.push((output_attributes::LITERAL_DATA_MODIFIED.name.to_string(), literal_data_header.created().to_string()));
                }
                session.write(&mut ff, &data_vec);
                session.transfer(ff, &SUCCESS.name);
                Ok(OnTriggerResult::Ok)
            } else {
                logger.warn("Failed to serialize decrypted message");
                session.transfer(ff, &FAILURE.name);
                Ok(OnTriggerResult::Ok)
            }
        } else {
            session.transfer(ff, &FAILURE.name);
            Ok(OnTriggerResult::Ok)
        }
    }
}

impl CalculateMetrics for DecryptContent {}

#[cfg(test)]
mod tests;

#[cfg(not(test))]
mod processor_definition;
