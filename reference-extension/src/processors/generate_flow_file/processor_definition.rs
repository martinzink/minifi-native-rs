use super::properties::*;
use super::{GenerateFlowFile, relationships};
use minifi_native::{
    CffiLogger, ProcessorDefinition, ProcessorInputRequirement, RegisterableProcessor,
};

impl RegisterableProcessor for GenerateFlowFile<CffiLogger> {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(ProcessorDefinition::<GenerateFlowFile<CffiLogger>>::new(
            "rs::GenerateFlowFileRs",
            "This processor creates FlowFiles with random data or custom content. GenerateFlowFile is useful for load testing, configuration, and simulation.",
            ProcessorInputRequirement::Forbidden,
            false,
            false,
            &[],
            &[relationships::SUCCESS],
            &[
                FILE_SIZE,
                BATCH_SIZE,
                DATA_FORMAT,
                UNIQUE_FLOW_FILES,
                CUSTOM_TEXT,
            ],
        ))
    }
}
