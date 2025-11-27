mod processors;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!([
    processors::generate_flow_file::GenerateFlowFile,
    processors::get_file::GetFile,
    processors::kamikaze_processor::KamikazeProcessor,
    processors::log_attribute::LogAttribute,
    processors::put_file::PutFile
]);
