mod api;
mod c_ffi;
mod mock;

pub use api::raw::raw_controller_service::RawControllerService; // TODO(mzink) replace with more user friendly API

pub use api::component_definition_traits::{
    ComponentIdentifier, ControllerServiceDefinition, ProcessorDefinition,
};
pub use api::controller_service::{ControllerService, EnableControllerService};
pub use api::errors::MinifiError;
pub use api::processor_traits::{CalculateMetrics, Schedule};

// TODO(mzink) clean this up
pub use api::{
    Concurrent, ConstTrigger, Content, ControllerServiceContext, Exclusive, FlowFile,
    FlowFileTransform, FlowFileTransformer, HasRawProcessorDefinition, LogLevel, Logger,
    MultiThreadedProcessor, MutTrigger, OnTriggerResult, OutputAttribute, ProcessContext,
    ProcessSession, ProcessorInputRequirement, Property, RawMultiThreadedTrigger, RawProcessor,
    RawSingleThreadedTrigger, Relationship, SingleThreadedProcessor, StandardPropertyValidator,
    TransformedFlowFile,
};
pub use c_ffi::{
    CffiControllerServiceDefinition, CffiControllerServiceList, CffiLogger, CffiProcessorList,
    DynRawControllerServiceDefinition, DynRawProcessorDefinition, RawProcessorDefinition,
    RawRegisterableProcessor, RegisterableControllerService, StaticStrAsMinifiCStr,
};
pub use mock::{
    MockControllerServiceContext, MockFlowFile, MockLogger, MockProcessContext, MockProcessSession,
    StdLogger,
};

pub use minifi_macros as macros;
pub use minifi_native_sys as sys;
use minifi_native_sys::{
    MINIFI_API_MAJOR_VERSION, MINIFI_API_MINOR_VERSION, MINIFI_API_PATCH_VERSION,
};

#[unsafe(no_mangle)]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".rodata"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__const"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".rdata"))]
pub static MINIFI_C_API_VERSION: &str = const_format::concatcp!(
    "MINIFI_API_VERSION=[",
    MINIFI_API_MAJOR_VERSION,
    ".",
    MINIFI_API_MINOR_VERSION,
    ".",
    MINIFI_API_PATCH_VERSION,
    "]"
);

#[macro_export]
macro_rules! declare_minifi_extension {
    (
        processors: [ $($proc:path),* $(,)? ],
        controllers: [ $($ctrl:path),* $(,)? ]
    ) => {

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub extern "C" fn MinifiInitExtension(
            _config: *mut minifi_native::sys::MinifiConfig,
        ) -> *mut minifi_native::sys::MinifiExtension {

            use minifi_native::StaticStrAsMinifiCStr;

            unsafe {
                let mut processor_list = minifi_native::CffiProcessorList::new();

                $(
                    {
                        processor_list.add::<$proc>();
                    }
                )*

                let mut controller_list = minifi_native::CffiControllerServiceList::new();

                $(
                    {
                        controller_list.add::<$ctrl>();
                    }
                )*

                let extension_create_info = minifi_native::sys::MinifiExtensionCreateInfo {
                    name: env!("CARGO_PKG_NAME").as_minifi_c_type(),
                    version: env!("CARGO_PKG_VERSION").as_minifi_c_type(),
                    deinit: None,
                    user_data: std::ptr::null_mut(),
                    processors_count: processor_list.get_processor_count(),
                    processors_ptr: processor_list.get_processor_ptr(),
                    controller_services_count: controller_list.get_controller_service_count(),
                    controller_services_ptr: controller_list.get_controller_service_ptr(),
                };

                minifi_native::sys::MinifiCreateExtension_0_1(minifi_native::MINIFI_C_API_VERSION.as_minifi_c_type(), &extension_create_info)
            }
        }
    };
}
