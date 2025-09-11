use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};
use crate::processors::log_attribute::{properties, relationships, LogAttribute};

#[cfg_attr(test, allow(dead_code))]
fn log_attribute_definition() -> ProcessorDefinition<LogAttribute<CffiLogger>> {
    let mut simple_log_processor_definition =
        ProcessorDefinition::<LogAttribute<CffiLogger>>::new(
            "rust_reference_extension",
            "rs::LogAttributeRs",
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
fn register_log_attribute() {
    log_attribute_definition().register_class();
}
