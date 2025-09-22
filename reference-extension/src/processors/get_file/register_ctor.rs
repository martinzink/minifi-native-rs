use crate::processors::get_file::properties::*;
use crate::processors::get_file::{GetFile, relationships};
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn get_file_definition() -> ProcessorDefinition<GetFile<CffiLogger>> {
    let mut simple_log_processor_definition = ProcessorDefinition::<GetFile<CffiLogger>>::new(
        "rust_reference_extension",
        "rs::GetFileRs",
        "Creates FlowFiles from files in a directory. MiNiFi will ignore files for which it doesn't have read permissions.",
    );

    simple_log_processor_definition.is_single_threaded = false;
    simple_log_processor_definition.input_requirement = ProcessorInputRequirement::Forbidden;
    simple_log_processor_definition.supports_dynamic_properties = false;
    simple_log_processor_definition.supports_dynamic_relationships = false;
    simple_log_processor_definition.relationships = &[relationships::SUCCESS];
    simple_log_processor_definition.properties = &[
        DIRECTORY,
        RECURSE,
        KEEP_SOURCE_FILE,
        MIN_AGE,
        MAX_AGE,
        MIN_SIZE,
        MAX_SIZE,
        IGNORE_HIDDEN_FILES,
        BATCH_SIZE,
    ];
    simple_log_processor_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_get_file() {
    get_file_definition().register_class();
}
