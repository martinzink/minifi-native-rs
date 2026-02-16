use super::DummyProcessorRs;
use super::properties::*;
use minifi_native::{
    OutputAttribute, ProcessorDefinition, ProcessorInputRequirement, Property, Relationship,
};

impl ProcessorDefinition for DummyProcessorRs {
    const DESCRIPTION: &'static str = "Processor to test Controller Service API";
    const INPUT_REQUIREMENT: ProcessorInputRequirement = ProcessorInputRequirement::Forbidden;
    const SUPPORTS_DYNAMIC_PROPERTIES: bool = false;
    const SUPPORTS_DYNAMIC_RELATIONSHIPS: bool = false;
    const OUTPUT_ATTRIBUTES: &'static [OutputAttribute] = &[];
    const RELATIONSHIPS: &'static [Relationship] = &[];
    const PROPERTIES: &'static [Property] = &[CONTROLLER_SERVICE];
}
