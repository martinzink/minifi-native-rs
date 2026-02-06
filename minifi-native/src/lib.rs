mod api;
mod c_ffi;
mod mock;

pub use api::{
    Concurrent, ConstTriggerable, ControllerService, ControllerServiceContext, DefaultLogger,
    Exclusive, FlowFile, HasProcessorDefinition, LogLevel, Logger, MetricsProvider, MinifiError,
    MultiThreadedProcessor, MutTriggerable, OnTriggerResult, OutputAttribute, ProcessContext,
    ProcessSession, ProcessorInputRequirement, Property, RawMultiThreadedTrigger, RawProcessor,
    RawSingleThreadedTrigger, Relationship, Schedulable, SingleThreadedProcessor,
    StandardPropertyValidator,
};
pub use c_ffi::{
    CffiControllerServiceList, CffiLogger, CffiProcessorList, ControllerServiceDefinition,
    DynControllerServiceDefinition, DynProcessorDefinition, ProcessorDefinition,
    RawRegisterableProcessor, RegisterableControllerService, StaticStrAsMinifiCStr,
};
pub use mock::{
    MockControllerServiceContext, MockFlowFile, MockLogger, MockProcessContext, MockProcessSession,
    StdLogger,
};

pub use minifi_native_sys as sys;
use minifi_native_sys::{
    MINIFI_API_MAJOR_VERSION, MINIFI_API_MINOR_VERSION, MINIFI_API_PATCH_VERSION,
};

#[unsafe(no_mangle)]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".rodata"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__const"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".rdata"))]
pub static API_VERSION_STRING: &str = const_format::concatcp!(
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
        pub extern "C" fn InitExtension(
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

                minifi_native::sys::MinifiCreateExtension(minifi_native::API_VERSION_STRING.as_minifi_c_type(), &extension_create_info)
            }
        }
    };
}
