use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};
use super::{relationships, GenerateFlowFile};
use super::properties::*;

#[cfg_attr(test, allow(dead_code))]
fn get_file_definition() -> ProcessorDefinition<GenerateFlowFile<CffiLogger>> {
    let mut simple_log_processor_definition =
        ProcessorDefinition::<GenerateFlowFile<CffiLogger>>::new(
            "rust_reference_extension",
            "rs::GenerateFlowFileRs",
            "This processor creates FlowFiles with random data or custom content. GenerateFlowFile is useful for load testing, configuration, and simulation.",
        );

    simple_log_processor_definition.is_single_threaded = false;
    simple_log_processor_definition.input_requirement = ProcessorInputRequirement::Forbidden;
    simple_log_processor_definition.supports_dynamic_properties = false;
    simple_log_processor_definition.supports_dynamic_relationships = false;
    simple_log_processor_definition.relationships = &[relationships::SUCCESS];
    simple_log_processor_definition.properties = &[FILE_SIZE, BATCH_SIZE, DATA_FORMAT, UNIQUE_FLOW_FILES, CUSTOM_TEXT];
    simple_log_processor_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_get_file() {
    get_file_definition().register_class();
}
