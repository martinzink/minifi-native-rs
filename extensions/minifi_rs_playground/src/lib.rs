mod controller_services;
mod processors;

#[cfg(not(test))]
use minifi_native::{ComplexProcessorType, Concurrent, Exclusive, FlowFileSourceProcessorType};

#[cfg(not(test))]
minifi_native::declare_minifi_extension!(
processors: [
    minifi_native::Processor<processors::generate_flow_file::GenerateFlowFileRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<processors::log_attribute::LogAttributeRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<processors::get_file::GetFileRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<processors::kamikaze_processor::KamikazeProcessorRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<processors::lorem_ipsum_cs_user::LoremIpsumCSUser, FlowFileSourceProcessorType, Concurrent>,
    minifi_native::Processor<processors::put_file::PutFileRs, ComplexProcessorType, Exclusive>,
],
controllers: [
    minifi_native::ControllerService<controller_services::lorem_ipsum_controller_service::LoremIpsumControllerService>
]);
