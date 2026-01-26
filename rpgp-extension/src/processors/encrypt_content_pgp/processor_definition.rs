use super::{EncryptContentPGP, relationships, properties};
use minifi_native::{
    CffiLogger, ProcessorDefinition, ProcessorInputRequirement, RegisterableProcessor,
};

impl RegisterableProcessor for EncryptContentPGP<CffiLogger> {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(ProcessorDefinition::<EncryptContentPGP<CffiLogger>>::new(
            "rs::EncryptContentPGP",
            "Encrypt contents using OpenPGP. The processor reads input and detects OpenPGP messages to avoid unnecessary additional wrapping in Literal Data packets.",
            ProcessorInputRequirement::Required,
            false,
            false,
            &[],
            &[relationships::SUCCESS, relationships::FAILURE],
            &[properties::FILE_ENCODING, properties::PASSPHRASE, properties::PUBLIC_KEY_SEARCH, properties::SYMMETRIC_KEY_ALGORITHM
            ],
        ))
    }
}
