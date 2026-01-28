use super::properties::*;
use super::{DummyProcessor};
use minifi_native::{
    ProcessorDefinition, ProcessorInputRequirement, RegisterableProcessor,
};
use crate::processors::dummy_processor::relationships::SUCCESS;

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
            &[
                CONTROLLER_SERVICE
            ],
        ))
    }
}
