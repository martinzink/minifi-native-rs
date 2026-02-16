use super::*;
use minifi_native::{HasRawProcessorDefinition, OutputAttribute, ProcessorDefinition, ProcessorInputRequirement, Property, RawProcessorDefinition, Relationship, SingleThreadedProcessor};

#[cfg(windows)]
const fn get_properties() -> &'static [Property] {
    &[
        properties::DIRECTORY,
        properties::CONFLICT_RESOLUTION,
        properties::CREATE_DIRS,
        properties::MAX_FILE_COUNT,
    ]
}

#[cfg(unix)]
const fn get_properties() -> &'static [Property] {
    &[
        properties::DIRECTORY,
        properties::CONFLICT_RESOLUTION,
        properties::CREATE_DIRS,
        properties::MAX_FILE_COUNT,
        unix_only_properties::PERMISSIONS,
        unix_only_properties::DIRECTORY_PERMISSIONS,
    ]
}

impl HasRawProcessorDefinition for PutFileRs {
    fn get_definition() -> Box<dyn minifi_native::DynRawProcessorDefinition> {
        Box::new(
            RawProcessorDefinition::<SingleThreadedProcessor<PutFileRs>>::new(
                "rs::PutFileRs",
                "Writes the contents of a FlowFile to the local file system.",
                ProcessorInputRequirement::Required,
                false,
                false,
                &[],
                &[relationships::SUCCESS, relationships::FAILURE],
                get_properties(),
            ),
        )
    }
}

impl ProcessorDefinition for PutFileRs {
    const DESCRIPTION: &'static str = "Writes the contents of a FlowFile to the local file system.";
    const INPUT_REQUIREMENT: ProcessorInputRequirement = ProcessorInputRequirement::Required;
    const SUPPORTS_DYNAMIC_PROPERTIES: bool = false;
    const SUPPORTS_DYNAMIC_RELATIONSHIPS: bool = false;
    const OUTPUT_ATTRIBUTES: &'static [OutputAttribute] = &[];
    const RELATIONSHIPS: &'static [Relationship] = &[relationships::SUCCESS, relationships::FAILURE];
    const PROPERTIES: &'static [Property] = get_properties();
}
