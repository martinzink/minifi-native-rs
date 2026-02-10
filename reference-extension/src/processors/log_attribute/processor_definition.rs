use crate::processors::log_attribute::properties::*;
use crate::processors::log_attribute::{LogAttribute, relationships};
use minifi_native::{
    HasRawProcessorDefinition, MultiThreadedProcessor, RawProcessorDefinition, ProcessorInputRequirement,
};

impl HasRawProcessorDefinition for LogAttribute {
    fn get_definition() -> Box<dyn minifi_native::DynRawProcessorDefinition> {
        Box::new(
            RawProcessorDefinition::<MultiThreadedProcessor<LogAttribute>>::new(
                "rs::LogAttributeRs",
                "Logs attributes of flow files in the MiNiFi application log.",
                ProcessorInputRequirement::Required,
                false,
                false,
                &[],
                &[relationships::SUCCESS],
                &[
                    LOG_LEVEL,
                    ATTRIBUTES_TO_LOG,
                    ATTRIBUTES_TO_IGNORE,
                    LOG_PAYLOAD,
                    LOG_PREFIX,
                    FLOW_FILES_TO_LOG,
                    HEX_ENCODE_PAYLOAD,
                ],
            ),
        )
    }
}
