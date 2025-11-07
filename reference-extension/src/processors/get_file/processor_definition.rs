use crate::processors::get_file::properties::*;
use crate::processors::get_file::{GetFile, relationships};
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
pub(crate) fn processor_definition() -> ProcessorDefinition<GetFile<CffiLogger>> {
    ProcessorDefinition::<GetFile<CffiLogger>>::new(
        "rs::GetFileRs",
        "Creates FlowFiles from files in a directory. MiNiFi will ignore files for which it doesn't have read permissions.",
        ProcessorInputRequirement::Forbidden,
        false,
        false,
        &[relationships::SUCCESS],
        &[
            DIRECTORY,
            RECURSE,
            KEEP_SOURCE_FILE,
            MIN_AGE,
            MAX_AGE,
            MIN_SIZE,
            MAX_SIZE,
            IGNORE_HIDDEN_FILES,
            BATCH_SIZE,
        ],
    )
}
