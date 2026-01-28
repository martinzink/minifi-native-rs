use minifi_native::{
    Concurrent, ConcurrentOnTrigger, LogLevel, Logger, MinifiError, OnTriggerResult,
    ProcessContext, ProcessSession, Processor,
};

mod processor_definition;
mod properties;
mod relationships;
mod tests;

use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
enum SymmetricKeyAlgorithm {
    Aes_128,
    Aes_192,
    Aes_256,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
enum FileEncoding {
    Ascii,
    Binary
}

#[derive(Debug)]
struct ScheduledMembers {
    symmetric_key_algorithm: SymmetricKeyAlgorithm,
    file_encoding: FileEncoding,
}

impl ScheduledMembers {
    fn on_trigger<P: ProcessContext, S: ProcessSession>(
        &self,
        context: &mut P,
        session: &mut S,
    ) -> Result<OnTriggerResult, MinifiError> {
        if let Some(flow_file) = session.get() {
            
            Ok(OnTriggerResult::Ok)
        } else {
            Ok(OnTriggerResult::Yield)
        }
    }
}

#[derive(Debug)]
pub(crate) struct EncryptContentPGP {
    logger: L,
    scheduled_members: Option<ScheduledMembers>,
}

impl Processor for EncryptContentPGP {
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
        let symmetric_key_algorithm = context
            .get_property(&properties::SYMMETRIC_KEY_ALGORITHM, None)?
            .expect("required property")
            .parse::<SymmetricKeyAlgorithm>()?;

        let file_encoding = context
            .get_property(&properties::FILE_ENCODING, None)?
            .expect("required property")
            .parse::<FileEncoding>()?;

        self.scheduled_members = Some(ScheduledMembers {
            symmetric_key_algorithm,
            file_encoding
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
