use crate::processors::get_file::properties::*;
use crate::processors::get_file::{GetFile, relationships};
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn get_file_definition() -> ProcessorDefinition<GetFile<CffiLogger>> {
    let mut get_file_def = ProcessorDefinition::<GetFile<CffiLogger>>::new(
        "rust_reference_extension",
        "rs::GetFileRs",
        "Creates FlowFiles from files in a directory. MiNiFi will ignore files for which it doesn't have read permissions.",
    );

    get_file_def.input_requirement = ProcessorInputRequirement::Forbidden;
    get_file_def.supports_dynamic_properties = false;
    get_file_def.supports_dynamic_relationships = false;
    get_file_def.relationships = &[relationships::SUCCESS];
    get_file_def.properties = &[
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
    get_file_def
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_get_file() {
    get_file_definition().register_class();
}
