mod processors;

use minifi_native::sys::{
    MINIFI_API_MAJOR_VERSION, MINIFI_API_MINOR_VERSION, MINIFI_API_PATCH_VERSION,
};

use const_format::concatcp;

#[used]
static API_VERSION_STRING: &str = concatcp!(
    "MINIFI_API_VERSION=[",
    MINIFI_API_MAJOR_VERSION,
    ".",
    MINIFI_API_MINOR_VERSION,
    ".",
    MINIFI_API_PATCH_VERSION,
    "]"
);
