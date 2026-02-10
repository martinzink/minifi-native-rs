use super::{DecryptContent, output_attributes, properties, relationships};
use minifi_native::{
    FlowFileTransformer, HasProcessorDefinition, ProcessorDefinition, ProcessorInputRequirement,
};

impl HasProcessorDefinition for DecryptContent {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(
            ProcessorDefinition::<FlowFileTransformer<DecryptContent>>::new(
                "rs::DecryptContentPGP",
                "Decrypt contents of OpenPGP messages. Using the Packaged Decryption Strategy preserves OpenPGP encoding to support subsequent signature verification.",
                ProcessorInputRequirement::Required,
                false,
                false,
                &[
                    output_attributes::LITERAL_DATA_FILENAME,
                    output_attributes::LITERAL_DATA_MODIFIED,
                ],
                &[relationships::SUCCESS, relationships::FAILURE],
                &[
                    properties::DECRYPTION_STRATEGY,
                    properties::SYMMETRIC_PASSWORD,
                    properties::PRIVATE_KEY_SERVICE,
                ],
            ),
        )
    }
}
