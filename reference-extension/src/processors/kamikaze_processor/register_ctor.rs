use super::*;
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn kamikaze_processor_definition() -> ProcessorDefinition<KamikazeProcessor<CffiLogger>> {
    let mut simple_log_processor_definition =
        ProcessorDefinition::<KamikazeProcessor<CffiLogger>>::new(
            "rust_reference_extension",
            "rs::KamikazeProcessorRs",
            "This processor can panic in on_trigger and on_schedule calls based on configuration. Only for testing purposes.",
        );

    simple_log_processor_definition.input_requirement = ProcessorInputRequirement::Required;
    simple_log_processor_definition.supports_dynamic_properties = false;
    simple_log_processor_definition.supports_dynamic_relationships = false;
    simple_log_processor_definition.relationships = &[relationships::SUCCESS];
    simple_log_processor_definition.properties = &[
        properties::ON_SCHEDULE_BEHAVIOUR,
        properties::ON_TRIGGER_BEHAVIOUR,
        properties::WRITE_BEHAVIOUR,
        properties::READ_BEHAVIOUR,
    ];
    simple_log_processor_definition
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_kamikaze_processor() {
    kamikaze_processor_definition().register_class();
}
