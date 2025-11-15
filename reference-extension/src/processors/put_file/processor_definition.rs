use super::*;
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement, Property};

#[cfg_attr(test, allow(dead_code))]
pub(crate) fn processor_definition() -> ProcessorDefinition<PutFile<CffiLogger>> {
    let properties: &'static [Property] = if cfg!(unix) {
        &[
            properties::DIRECTORY,
            properties::CONFLICT_RESOLUTION,
            properties::CREATE_DIRS,
            properties::MAX_FILE_COUNT,
            unix_only_properties::PERMISSIONS,
            unix_only_properties::DIRECTORY_PERMISSIONS,
        ]
    } else {
        &[
            properties::DIRECTORY,
            properties::CONFLICT_RESOLUTION,
            properties::CREATE_DIRS,
            properties::MAX_FILE_COUNT,
        ]
    };

    ProcessorDefinition::<PutFile<CffiLogger>>::new(
        "rs::PutFileRs",
        "Writes the contents of a FlowFile to the local file system.",
        ProcessorInputRequirement::Required,
        false,
        false,
        &[],
        &[relationships::SUCCESS, relationships::FAILURE],
        properties,
    )
}
