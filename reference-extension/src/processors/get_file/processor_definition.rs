use crate::processors::get_file::output_attributes::{
    ABSOLUTE_PATH_OUTPUT_ATTRIBUTE, FILENAME_OUTPUT_ATTRIBUTE,
};
use crate::processors::get_file::properties::*;
use crate::processors::get_file::{GetFile, relationships};
use minifi_native::{
    HasRawProcessorDefinition, MultiThreadedProcessor, ProcessorInputRequirement,
    RawProcessorDefinition,
};

impl HasRawProcessorDefinition for GetFile {
    fn get_definition() -> Box<dyn minifi_native::DynRawProcessorDefinition> {
        Box::new(
            RawProcessorDefinition::<MultiThreadedProcessor<GetFile>>::new(
                "rs::GetFileRs",
                "Creates FlowFiles from files in a directory. MiNiFi will ignore files for which it doesn't have read permissions.",
                ProcessorInputRequirement::Forbidden,
                false,
                false,
                &[ABSOLUTE_PATH_OUTPUT_ATTRIBUTE, FILENAME_OUTPUT_ATTRIBUTE],
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
            ),
        )
    }
}
