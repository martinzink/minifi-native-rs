mod processors;

#[unsafe(no_mangle)]
#[cfg(not(test))]
#[allow(non_snake_case)]
pub extern "C" fn InitExtension(
    _config: *mut minifi_native::sys::MinifiConfig,
) -> *mut minifi_native::sys::MinifiExtension {
    use minifi_native::StaticStrAsMinifiCStr;

    unsafe {
        let mut processor_list = minifi_native::CffiProcessorList::new();
        processor_list.add::<processors::GenerateFlowFile>();
        processor_list.add::<processors::GetFile>();
        processor_list.add::<processors::KamikazeProcessor>();
        processor_list.add::<processors::LogAttribute>();
        processor_list.add::<processors::PutFile>();

        // processor_list must outlive this MinifiExtensionCreateInfo call
        let extension_create_info = minifi_native::sys::MinifiExtensionCreateInfo {
            name: "Rust Reference Extension".as_minifi_c_type(),
            version: env!("CARGO_PKG_VERSION").as_minifi_c_type(),
            deinit: None,
            user_data: std::ptr::null_mut(),
            processors_count: processor_list.get_processor_count(),
            processors_ptr: processor_list.get_processor_ptr(),
        };
        minifi_native::sys::MinifiCreateExtension(&extension_create_info)
    }
}
