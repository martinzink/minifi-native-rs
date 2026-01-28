use super::DummyProcessor;
use super::properties::*;
use crate::processors::dummy_processor::relationships::SUCCESS;
use minifi_native::{ProcessorDefinition, ProcessorInputRequirement, RegisterableProcessor};

impl RegisterableProcessor for DummyProcessor {
    fn get_definition() -> Box<dyn minifi_native::DynProcessorDefinition> {
        Box::new(ProcessorDefinition::<DummyProcessor>::new(
            "rs::DummyProcessorRs",
            "Processor to test Controller Service API",
            ProcessorInputRequirement::Forbidden,
            false,
            false,
            &[],
            &[SUCCESS],
            &[CONTROLLER_SERVICE],
        ))
    }
}
