mod controller_services;
mod processors;

use crate::controller_services::dummy_controller_service::DummyControllerService;
use crate::controller_services::lorem_ipsum_controller_service::LoremIpsumControllerService;
use crate::processors::asciify_german::AsciifyGerman;
use crate::processors::count_actual_logging::CountActualLogging;
use crate::processors::generate_flow_file::GenerateFlowFileRs;
use crate::processors::get_file::GetFileRs;
use crate::processors::kamikaze_processor::KamikazeProcessorRs;
use crate::processors::log_attribute::LogAttributeRs;
use crate::processors::lorem_ipsum_cs_user::LoremIpsumCSUser;
use crate::processors::put_file::PutFileRs;
use minifi_native::{
    ComplexProcessorType, Concurrent, Exclusive, FlowFileSourceProcessorType,
    FlowFileStreamTransformProcessorType, FlowFileTransformProcessorType,
};

minifi_native::declare_minifi_extension!(
processors: [
    minifi_native::Processor<GenerateFlowFileRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<LogAttributeRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<GetFileRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<KamikazeProcessorRs, ComplexProcessorType, Concurrent>,
    minifi_native::Processor<LoremIpsumCSUser, FlowFileSourceProcessorType, Concurrent>,
    minifi_native::Processor<PutFileRs, FlowFileTransformProcessorType, Concurrent>,
    minifi_native::Processor<AsciifyGerman, FlowFileStreamTransformProcessorType, Concurrent>,
    minifi_native::Processor<CountActualLogging, ComplexProcessorType, Exclusive>,
],
controllers: [
    minifi_native::ControllerService<LoremIpsumControllerService>,
    minifi_native::ControllerService<DummyControllerService>,
]);
