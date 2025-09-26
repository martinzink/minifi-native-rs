use super::properties::*;
use super::{GenerateFlowFile, relationships};
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn generate_flow_file_definition() -> ProcessorDefinition<GenerateFlowFile<CffiLogger>> {
    let mut generate_flow_file_definition =
        ProcessorDefinition::<GenerateFlowFile<CffiLogger>>::new(
            "rust_reference_extension",
            "rs::GenerateFlowFileRs",
            "This processor creates FlowFiles with random data or custom content. GenerateFlowFile is useful for load testing, configuration, and simulation.",
        );

    generate_flow_file_definition.input_requirement = ProcessorInputRequirement::Forbidden;
    generate_flow_file_definition.supports_dynamic_properties = false;
    generate_flow_file_definition.supports_dynamic_relationships = false;
    generate_flow_file_definition.relationships = &[relationships::SUCCESS];
    generate_flow_file_definition.properties = &[
        FILE_SIZE,
        BATCH_SIZE,
        DATA_FORMAT,
        UNIQUE_FLOW_FILES,
        CUSTOM_TEXT,
    ];
    generate_flow_file_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_generate_flow_file() {
    generate_flow_file_definition().register_class();
}
