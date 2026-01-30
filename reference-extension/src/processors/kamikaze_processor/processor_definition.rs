use super::*;
use minifi_native::{HasProcessorDefinition, MultiThreadedProcessor, ProcessorDefinition, ProcessorInputRequirement};

impl HasProcessorDefinition for KamikazeProcessor {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(ProcessorDefinition::<MultiThreadedProcessor<KamikazeProcessor>>::new(
            "rs::KamikazeProcessorRs",
            "This processor can fail or panic in on_trigger and on_schedule calls based on configuration. Only for testing purposes.",
            ProcessorInputRequirement::Allowed,
            false,
            false,
            &[],
            &[relationships::SUCCESS],
            &[
                properties::ON_SCHEDULE_BEHAVIOUR,
                properties::ON_TRIGGER_BEHAVIOUR,
                properties::READ_BEHAVIOUR,
            ],
        ))
    }
}
