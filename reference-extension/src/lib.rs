mod processors;

use minifi_native::sys::{MINIFI_API_MAJOR_VERSION, MINIFI_API_MINOR_VERSION, MINIFI_API_PATCH_VERSION};

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
pub extern "C" fn InitExtension(_config: *mut minifi_native::sys::MinifiConfig) -> *mut minifi_native::sys::MinifiExtension {
    use minifi_native::StaticStrAsMinifiCStr;
    use minifi_native::sys::{MinifiProcessorClassDescription, MinifiExtensionCreateInfo, MinifiCreateExtension};
    unsafe {
        let mut processor_vec: Vec<MinifiProcessorClassDescription> = vec![];
        let gen_ff = processors::generate_flow_file::c_ffi_class_description::processor_class_description();
        processor_vec.push(gen_ff.class_description());

        let get_file = processors::get_file::c_ffi_class_description::processor_class_description();
        processor_vec.push(get_file.class_description());

        let kamikaze_processor = processors::kamikaze_processor::c_ffi_class_description::processor_class_description();
        processor_vec.push(kamikaze_processor.class_description());

        let log_attribute = processors::log_attribute::c_ffi_class_description::processor_class_description();
        processor_vec.push(log_attribute.class_description());

        let put_file = processors::put_file::c_ffi_class_description::processor_class_description();
        processor_vec.push(put_file.class_description());

        let extension_create_info: MinifiExtensionCreateInfo = MinifiExtensionCreateInfo{
            name: "Rust Reference Extension".as_minifi_c_type(),
            version: env!("CARGO_PKG_VERSION").as_minifi_c_type(),
            deinit: None,
            user_data: std::ptr::null_mut(),
            processors_count: processor_vec.len() as u32,
            processors_ptr: processor_vec.as_ptr(),
        };
        MinifiCreateExtension(&extension_create_info)
    }
}