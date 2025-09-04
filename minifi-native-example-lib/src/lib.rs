mod processors;

use minifi_native::sys::{
    MINIFI_API_MAJOR_VERSION, MINIFI_API_MINOR_VERSION, MINIFI_API_PATCH_VERSION,
};

use const_format::concatcp;

#[unsafe(no_mangle)]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".rodata"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__const"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".rdata"))]
static API_VERSION_STRING: &str = concatcp!(
    "MINIFI_API_VERSION=[",
    MINIFI_API_MAJOR_VERSION,
    ".",
    MINIFI_API_MINOR_VERSION,
    ".",
    MINIFI_API_PATCH_VERSION,
    "]"
);
