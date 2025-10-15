use super::*;
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn put_file_definition() -> ProcessorDefinition<PutFile<CffiLogger>> {
    let mut put_file_definition = ProcessorDefinition::<PutFile<CffiLogger>>::new(
        "rust_reference_extension",
        "rs::PutFileRs",
        "Writes the contents of a FlowFile to the local file system.",
    );

    put_file_definition.input_requirement = ProcessorInputRequirement::Required;
    put_file_definition.supports_dynamic_properties = false;
    put_file_definition.supports_dynamic_relationships = false;
    put_file_definition.relationships = &[relationships::SUCCESS, relationships::FAILURE];
    put_file_definition.properties = &[
        properties::DIRECTORY,
        properties::CONFLICT_RESOLUTION,
        properties::CREATE_DIRS,
        properties::MAX_FILE_COUNT,
    ];
    put_file_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_put_file() {
    put_file_definition().register_class();
}
