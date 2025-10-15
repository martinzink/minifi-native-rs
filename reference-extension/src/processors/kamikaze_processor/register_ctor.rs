use super::*;
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
fn kamikaze_processor_definition() -> ProcessorDefinition<KamikazeProcessor<CffiLogger>> {
    let mut kamikaze_processor_def = ProcessorDefinition::<KamikazeProcessor<CffiLogger>>::new(
        "rust_reference_extension",
        "rs::KamikazeProcessorRs",
        "This processor can fail or panic in on_trigger and on_schedule calls based on configuration. Only for testing purposes.",
    );

    kamikaze_processor_def.input_requirement = ProcessorInputRequirement::Allowed;
    kamikaze_processor_def.supports_dynamic_properties = false;
    kamikaze_processor_def.supports_dynamic_relationships = false;
    kamikaze_processor_def.relationships = &[relationships::SUCCESS];
    kamikaze_processor_def.properties = &[
        properties::ON_SCHEDULE_BEHAVIOUR,
        properties::ON_TRIGGER_BEHAVIOUR,
        properties::WRITE_BEHAVIOUR,
        properties::READ_BEHAVIOUR,
    ];
    kamikaze_processor_def
}

#[ctor::ctor]
#[unsafe(no_mangle)]
fn register_kamikaze_processor() {
    kamikaze_processor_definition().register_class();
}
