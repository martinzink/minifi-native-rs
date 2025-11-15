use super::*;
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
pub(crate) fn processor_definition() -> ProcessorDefinition<KamikazeProcessor<CffiLogger>> {
    ProcessorDefinition::<KamikazeProcessor<CffiLogger>>::new(
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
            properties::WRITE_BEHAVIOUR,
            properties::READ_BEHAVIOUR,
        ],
    )
}
