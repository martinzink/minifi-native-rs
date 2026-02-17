mod controller_services;
mod processors;

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(
processors: [
    minifi_native::MultiThreadedProcessor::<processors::generate_flow_file::GenerateFlowFileRs>,
    minifi_native::MultiThreadedProcessor::<processors::log_attribute::LogAttributeRs>,
    minifi_native::MultiThreadedProcessor::<processors::get_file::GetFileRs>,
    minifi_native::MultiThreadedProcessor::<processors::kamikaze_processor::KamikazeProcessorRs>,
    minifi_native::FlowFileSourceProcessor::<processors::lorem_ipsum_cs_user::LoremIpsumCSUser>,
    minifi_native::SingleThreadedProcessor::<processors::put_file::PutFileRs>,
],
controllers: [
    minifi_native::ControllerService::<controller_services::lorem_ipsum_controller_service::LoremIpsumControllerService>
]);
