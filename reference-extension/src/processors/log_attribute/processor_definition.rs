use crate::processors::log_attribute::properties::*;
use crate::processors::log_attribute::{LogAttribute, relationships};
use minifi_native::{
    CffiLogger, ProcessorDefinition, ProcessorInputRequirement, RegisterableProcessor,
};

impl RegisterableProcessor for LogAttribute<CffiLogger> {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(ProcessorDefinition::<LogAttribute<CffiLogger>>::new(
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
        ))
    }
}
