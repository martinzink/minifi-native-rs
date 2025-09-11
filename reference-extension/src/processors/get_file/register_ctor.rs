use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};
use crate::processors::get_file::{properties, relationships, GetFile};

#[cfg_attr(test, allow(dead_code))]
fn get_file_definition() -> ProcessorDefinition<GetFile<CffiLogger>> {
    let mut simple_log_processor_definition =
        ProcessorDefinition::<GetFile<CffiLogger>>::new(
            "rust_reference_extension",
            "rs::GetFileRs",
            "Logs attributes of flow files in the MiNiFi application log.",
        );

    simple_log_processor_definition.is_single_threaded = false;
    simple_log_processor_definition.input_requirement = ProcessorInputRequirement::Required;
    simple_log_processor_definition.supports_dynamic_properties = false;
    simple_log_processor_definition.supports_dynamic_relationships = false;
    simple_log_processor_definition.relationships = &[relationships::SUCCESS];
    simple_log_processor_definition.properties = &properties::PROPERTIES;
    simple_log_processor_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_get_file() {
    get_file_definition().register_class();
}
