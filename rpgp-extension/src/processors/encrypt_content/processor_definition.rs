use super::{EncryptContentPGP, output_attributes, properties, relationships};
use minifi_native::{
    HasProcessorDefinition, MultiThreadedProcessor, ProcessorDefinition, ProcessorInputRequirement,
};

impl HasProcessorDefinition for EncryptContentPGP {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(ProcessorDefinition::<
            MultiThreadedProcessor<EncryptContentPGP>,
        >::new(
            "rs::EncryptContentPGP",
            "Encrypt contents using OpenPGP. The processor reads input and detects OpenPGP messages to avoid unnecessary additional wrapping in Literal Data packets.",
            ProcessorInputRequirement::Required,
            false,
            false,
            &[output_attributes::FILE_ENCODING],
            &[relationships::SUCCESS, relationships::FAILURE],
            &[
                properties::FILE_ENCODING,
                properties::PASSPHRASE,
                properties::PUBLIC_KEY_SEARCH,
                properties::PUBLIC_KEY_SERVICE,
            ],
        ))
    }
}
