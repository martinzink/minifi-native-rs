use crate::processors::log_attribute::properties::*;
use crate::processors::log_attribute::{LogAttribute, relationships};
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn log_attribute_definition() -> ProcessorDefinition<LogAttribute<CffiLogger>> {
    let mut log_attribute_definition = ProcessorDefinition::<LogAttribute<CffiLogger>>::new(
        "rust_reference_extension",
        "rs::LogAttributeRs",
        "Logs attributes of flow files in the MiNiFi application log.",
    );

    log_attribute_definition.input_requirement = ProcessorInputRequirement::Required;
    log_attribute_definition.supports_dynamic_properties = false;
    log_attribute_definition.supports_dynamic_relationships = false;
    log_attribute_definition.relationships = &[relationships::SUCCESS];
    log_attribute_definition.properties = &[
        LOG_LEVEL,
        ATTRIBUTES_TO_LOG,
        ATTRIBUTES_TO_IGNORE,
        LOG_PAYLOAD,
        LOG_PREFIX,
        FLOW_FILES_TO_LOG,
        HEX_ENCODE_PAYLOAD,
    ];
    log_attribute_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_log_attribute() {
    log_attribute_definition().register_class();
}
