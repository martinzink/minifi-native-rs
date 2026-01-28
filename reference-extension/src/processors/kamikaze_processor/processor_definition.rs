use super::*;
use minifi_native::{
    ProcessorDefinition, ProcessorInputRequirement, RegisterableProcessor,
};

impl RegisterableProcessor for KamikazeProcessor {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(ProcessorDefinition::<KamikazeProcessor>::new(
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
