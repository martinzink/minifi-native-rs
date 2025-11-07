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

#[unsafe(no_mangle)]
#[cfg(not(test))]
#[allow(non_snake_case)]
pub extern "C" fn InitExtension(
    _config: *mut minifi_native::sys::MinifiConfig,
) -> *mut minifi_native::sys::MinifiExtension {
    use minifi_native::StaticStrAsMinifiCStr;
    use minifi_native::CffiProcessorList;
    use minifi_native::sys::{
        MinifiCreateExtension, MinifiExtensionCreateInfo,
    };
    unsafe {
        let mut processor_list = CffiProcessorList::new();
        processor_list.add_processor_definition(Box::new(processors::generate_flow_file::processor_definition::processor_definition()));
        processor_list.add_processor_definition(Box::new(processors::get_file::processor_definition::processor_definition()));
        processor_list.add_processor_definition(Box::new(processors::kamikaze_processor::processor_definition::processor_definition()));
        processor_list.add_processor_definition(Box::new(processors::put_file::processor_definition::processor_definition()));
        processor_list.add_processor_definition(Box::new(processors::log_attribute::processor_definition::processor_definition()));

        // processor_list must outlive this MinifiExtensionCreateInfo call
        let extension_create_info: MinifiExtensionCreateInfo = MinifiExtensionCreateInfo {
            name: "Rust Reference Extension".as_minifi_c_type(),
            version: env!("CARGO_PKG_VERSION").as_minifi_c_type(),
            deinit: None,
            user_data: std::ptr::null_mut(),
            processors_count: processor_list.get_processor_count(),
            processors_ptr: processor_list.get_processor_ptr(),
        };
        MinifiCreateExtension(&extension_create_info)
    }
}
