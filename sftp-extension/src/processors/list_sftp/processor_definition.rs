use super::properties::*;
use super::{ListSFTP, relationships};
use minifi_native::{CffiLogger, ProcessorDefinition, ProcessorInputRequirement};

#[cfg_attr(test, allow(dead_code))]
pub(crate) fn processor_definition() -> ProcessorDefinition<ListSFTP<CffiLogger>> {
    ProcessorDefinition::<ListSFTP<CffiLogger>>::new(
        "rs::ListSFTP",
        "Performs a listing of the files residing on an SFTP server. For each file that is found on the remote server, a new FlowFile will be created with the filename attribute set to the name of the file on the remote server. This can then be used in conjunction with FetchSFTP in order to fetch those files.",
        ProcessorInputRequirement::Forbidden,
        false,
        false,
        &[relationships::SUCCESS],
        &[
            FILE_SIZE,
            BATCH_SIZE,
            DATA_FORMAT,
            UNIQUE_FLOW_FILES,
            CUSTOM_TEXT,
        ],
    )
}
