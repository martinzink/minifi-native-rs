use super::properties::*;
use super::{GenerateFlowFileProcessor, relationships};
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
pub(crate) fn processor_class_description()
-> ProcessorDefinition<GenerateFlowFileProcessor<CffiLogger>> {
    ProcessorDefinition::<GenerateFlowFileProcessor<CffiLogger>>::new(
        "rs::GenerateFlowFileRs",
        "This processor creates FlowFiles with random data or custom content. GenerateFlowFile is useful for load testing, configuration, and simulation.",
        ProcessorInputRequirement::Forbidden,
        false,
        false,
        &[relationships::SUCCESS],
        &[
            FILE_SIZE,
            BATCH_SIZE,
            DATA_FORMAT,
            UNIQUE_FLOW_FILES,
            CUSTOM_TEXT,
        ],
    )
}
