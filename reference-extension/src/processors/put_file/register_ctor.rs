use super::*;
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn put_file_definition() -> ProcessorDefinition<PutFile<CffiLogger>> {
    let mut simple_log_processor_definition = ProcessorDefinition::<PutFile<CffiLogger>>::new(
        "rust_reference_extension",
        "rs::PutFileRs",
        "Writes the contents of a FlowFile to the local file system.",
    );

    simple_log_processor_definition.input_requirement = ProcessorInputRequirement::Required;
    simple_log_processor_definition.supports_dynamic_properties = false;
    simple_log_processor_definition.supports_dynamic_relationships = false;
    simple_log_processor_definition.relationships =
        &[relationships::SUCCESS, relationships::FAILURE];
    simple_log_processor_definition.properties = &[
        properties::DIRECTORY,
        properties::CONFLICT_RESOLUTION,
        properties::CREATE_DIRS,
        properties::MAX_FILE_COUNT,
    ];
    simple_log_processor_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_log_attribute() {
    put_file_definition().register_class();
}
