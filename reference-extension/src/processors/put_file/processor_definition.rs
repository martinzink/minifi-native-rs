use super::*;
use minifi_native::{
    HasRawProcessorDefinition, RawProcessorDefinition, ProcessorInputRequirement, Property,
    SingleThreadedProcessor,
};

#[cfg(windows)]
fn get_properties() -> &'static [Property] {
    &[
        properties::DIRECTORY,
        properties::CONFLICT_RESOLUTION,
        properties::CREATE_DIRS,
        properties::MAX_FILE_COUNT,
    ]
}

#[cfg(unix)]
fn get_properties() -> &'static [Property] {
    &[
        properties::DIRECTORY,
        properties::CONFLICT_RESOLUTION,
        properties::CREATE_DIRS,
        properties::MAX_FILE_COUNT,
        unix_only_properties::PERMISSIONS,
        unix_only_properties::DIRECTORY_PERMISSIONS,
    ]
}

impl HasRawProcessorDefinition for PutFile {
    fn get_definition() -> Box<dyn minifi_native::DynRawProcessorDefinition> {
        Box::new(
            RawProcessorDefinition::<SingleThreadedProcessor<PutFile>>::new(
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
