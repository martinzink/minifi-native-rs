mod output_attributes;
pub(crate) mod properties;
mod relationships;

use crate::controller_services::private_key_service::PGPPrivateKeyService;
use crate::processors::decrypt_content::properties::{PRIVATE_KEY_SERVICE, SYMMETRIC_PASSWORD};
use crate::processors::decrypt_content::relationships::{FAILURE, SUCCESS};
use minifi_native::macros::{ComponentIdentifier, DefaultMetrics};
use minifi_native::{
    FlowFileTransform, InputStream, Logger, MinifiError, ProcessContext, Schedule,
    TransformedFlowFile,
};
use pgp::composed::{Message, TheRing};
use std::collections::HashMap;
use std::fmt::Debug;
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE", const_into_str)]
enum DecryptionStrategy {
    Decrypted,
    Packaged,
}

#[derive(Debug, ComponentIdentifier, DefaultMetrics)]
pub(crate) struct DecryptContentPGP {
    decompress_data: bool,
    symmetric_password: Option<pgp::types::Password>,
}

impl Schedule for DecryptContentPGP {
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
            Ok(DecryptContentPGP {
                decompress_data: decryption_strategy == DecryptionStrategy::Decrypted,
                symmetric_password,
            })
        }
    }
}

impl DecryptContentPGP {
    fn decrypt_msg<'a, PC, L>(
        &self,
        msg: Message<'a>,
        ctx: &PC,
        _logger: &L,
    ) -> pgp::errors::Result<Message<'a>>
    where
        PC: ProcessContext,
        L: Logger,
    {
        let private_key_service = ctx
            .get_controller_service::<PGPPrivateKeyService>(&PRIVATE_KEY_SERVICE)
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

    fn extract_attributes_from_decrypted_message(
        decrypted_msg: &Message,
    ) -> HashMap<String, String> {
        let mut attributes_to_add = HashMap::new();
        if let Some(literal_data_header) = decrypted_msg.literal_data_header() {
            if let Ok(file_name) = str::from_utf8(literal_data_header.file_name()) {
                attributes_to_add.insert(
                    output_attributes::LITERAL_DATA_FILENAME.name.to_string(),
                    file_name.to_string(),
                );
            }
            attributes_to_add.insert(
                output_attributes::LITERAL_DATA_MODIFIED.name.to_string(),
                literal_data_header.created().as_secs().to_string(),
            );
        }
        attributes_to_add
    }
}

impl FlowFileTransform for DecryptContentPGP {
    fn transform<'ctx, 'stream, Context: ProcessContext, LoggerImpl: Logger>(
        &self,
        context: &'ctx mut Context,
        _flow_file: &Context::FlowFile,
        input_stream: &'stream mut dyn InputStream,
        logger: &LoggerImpl,
    ) -> Result<TransformedFlowFile<'stream>, MinifiError>
    where
        'ctx: 'stream,
    {
        let Ok(msg) = Message::from_reader(input_stream).map(|(msg, _header)| msg) else {
            logger.debug("No valid PGP message found");
            return Ok(TransformedFlowFile::route_without_changes(&FAILURE));
        };

        let Ok(mut decrypted_msg) = self.decrypt_msg(msg, context, logger) else {
            logger.debug("Failed to decrypt data");
            return Ok(TransformedFlowFile::route_without_changes(&FAILURE));
        };

        if self.decompress_data && decrypted_msg.is_compressed() {
            match decrypted_msg.decompress() {
                Ok(decompressed_data) => {
                    decrypted_msg = decompressed_data;
                }
                Err(e) => {
                    logger.debug(&format!("Failed to decompress data: {}", e));
                    return Ok(TransformedFlowFile::route_without_changes(&FAILURE));
                }
            }
        };

        let attributes_to_add = Self::extract_attributes_from_decrypted_message(&decrypted_msg);
        let Ok(new_content) = decrypted_msg.as_data_vec() else {
            logger.debug("Failed to extract raw data from decrypted message");
            return Ok(TransformedFlowFile::route_without_changes(&FAILURE));
        };

        Ok(TransformedFlowFile::new(
            &SUCCESS,
            Some(new_content),
            attributes_to_add,
        ))
    }
}

#[cfg(test)]
mod tests;

#[cfg(not(test))]
mod processor_definition;
