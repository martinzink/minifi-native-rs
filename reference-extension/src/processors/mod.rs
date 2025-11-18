pub(crate) mod generate_flow_file;
pub(crate) mod get_file;
pub(crate) mod kamikaze_processor;
pub(crate) mod log_attribute;
pub(crate) mod put_file;

#[cfg(not(test))]
pub(crate) type GenerateFlowFile = generate_flow_file::GenerateFlowFile<minifi_native::CffiLogger>;
#[cfg(not(test))]
pub(crate) type GetFile = get_file::GetFile<minifi_native::CffiLogger>;
#[cfg(not(test))]
pub(crate) type KamikazeProcessor =
    kamikaze_processor::KamikazeProcessor<minifi_native::CffiLogger>;
#[cfg(not(test))]
pub(crate) type LogAttribute = log_attribute::LogAttribute<minifi_native::CffiLogger>;
#[cfg(not(test))]
pub(crate) type PutFile = put_file::PutFile<minifi_native::CffiLogger>;
