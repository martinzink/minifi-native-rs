use super::DummyProcessor;
use super::properties::*;
use minifi_native::{
    HasRawProcessorDefinition, MultiThreadedProcessor, RawProcessorDefinition, ProcessorInputRequirement,
};

impl HasRawProcessorDefinition for DummyProcessor {
    fn get_definition() -> Box<dyn minifi_native::DynRawProcessorDefinition> {
        Box::new(
            RawProcessorDefinition::<MultiThreadedProcessor<DummyProcessor>>::new(
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
