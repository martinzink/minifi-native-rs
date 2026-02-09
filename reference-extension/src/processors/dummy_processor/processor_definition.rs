use super::DummyProcessor;
use super::properties::*;
use minifi_native::{
    HasProcessorDefinition, MultiThreadedProcessor, ProcessorDefinition, ProcessorInputRequirement,
};

impl HasProcessorDefinition for DummyProcessor {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(
            ProcessorDefinition::<MultiThreadedProcessor<DummyProcessor>>::new(
                "rs::DummyProcessorRs",
                "Processor to test Controller Service API",
                ProcessorInputRequirement::Forbidden,
                false,
                false,
                &[],
                &[],
                &[CONTROLLER_SERVICE],
            ),
        )
    }
}
