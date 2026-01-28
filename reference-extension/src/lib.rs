mod controller_services;
mod processors;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(
processors: [
    minifi_native::MultiThreadedProcessor::<processors::generate_flow_file::GenerateFlowFile>,
    minifi_native::MultiThreadedProcessor::<processors::log_attribute::LogAttribute>,
    minifi_native::MultiThreadedProcessor::<processors::get_file::GetFile>,
    minifi_native::MultiThreadedProcessor::<processors::kamikaze_processor::KamikazeProcessor>,
    minifi_native::MultiThreadedProcessor::<processors::dummy_processor::DummyProcessor>,
    minifi_native::SingleThreadedProcessor::<processors::put_file::PutFile>,
],
controllers: [
    controller_services::dummy_controller_service::DummyControllerService,
]);
