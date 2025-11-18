mod api;
mod c_ffi;
mod mock;

pub use api::{
    Concurrent, ConcurrentOnTrigger, Exclusive, ExclusiveOnTrigger, FlowFile, LogLevel, Logger,
    MinifiError, OnTriggerResult, OutputAttribute, ProcessContext, ProcessSession, Processor,
    ProcessorInputRequirement, Property, Relationship, StandardPropertyValidator,
};
pub use c_ffi::{
    CffiLogger, CffiProcessorList, DynProcessorDefinition, ProcessorDefinition,
    RegisterableProcessor, StaticStrAsMinifiCStr,
};
pub use mock::{MockFlowFile, MockLogger, MockProcessContext, MockProcessSession};

pub use minifi_native_sys as sys;

use crate::sys::{MINIFI_API_MAJOR_VERSION, MINIFI_API_MINOR_VERSION, MINIFI_API_PATCH_VERSION};

#[unsafe(no_mangle)]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".rodata"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__const"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".rdata"))]
static API_VERSION_STRING: &str = const_format::concatcp!(
    "MINIFI_API_VERSION=[",
    MINIFI_API_MAJOR_VERSION,
    ".",
    MINIFI_API_MINOR_VERSION,
    ".",
    MINIFI_API_PATCH_VERSION,
    "]"
);
