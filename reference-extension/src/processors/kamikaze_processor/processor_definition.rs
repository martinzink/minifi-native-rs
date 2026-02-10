use super::*;
use minifi_native::{
    HasRawProcessorDefinition, MultiThreadedProcessor, RawProcessorDefinition, ProcessorInputRequirement,
};

impl HasRawProcessorDefinition for KamikazeProcessor {
    fn get_definition() -> Box<dyn minifi_native::DynRawProcessorDefinition> {
        Box::new(RawProcessorDefinition::<
            MultiThreadedProcessor<KamikazeProcessor>,
        >::new(
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
