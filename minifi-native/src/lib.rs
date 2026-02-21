mod api;
pub mod c_ffi;
pub mod mock;

pub use api::errors::MinifiError;

pub use api::complex_processor::{ComplexProcessorType, ConstTrigger, MutTrigger};
pub use api::component_definition_traits::{
    ComponentIdentifier, ControllerServiceDefinition, ProcessorDefinition,
};
pub use api::controller_service::{ControllerService, EnableControllerService};
pub use api::flow_file_source::{FlowFileSource, FlowFileSourceProcessorType, GeneratedFlowFile};
pub use api::flow_file_transform::{
    FlowFileTransform, FlowFileTransformProcessorType, TransformedFlowFile,
};
pub use api::processor::{CalculateMetrics, Processor, Schedule};

pub use api::raw::raw_threading_model::{Concurrent, Exclusive};

pub use api::flow_file_content::Content;
pub use api::logger::{LogLevel, Logger};

pub use api::{
    ControllerServiceContext, FlowFile, InputStream, OnTriggerResult, OutputAttribute,
    ProcessContext, ProcessSession, ProcessorInputRequirement, Property, Relationship,
    StandardPropertyValidator,
};

pub use minifi_macros as macros;
pub use minifi_native_sys as sys;
pub use mock::{
    MockControllerServiceContext, MockFlowFile, MockLogger, MockProcessContext, MockProcessSession,
    StdLogger,
};

#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".rodata"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__const"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".rdata"))]
pub static MinifiApiVersion: &str = const_format::concatcp!(
    minifi_native_sys::MINIFI_API_MAJOR_VERSION,
    ".",
    minifi_native_sys::MINIFI_API_MINOR_VERSION,
    ".",
    minifi_native_sys::MINIFI_API_PATCH_VERSION,
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

            use minifi_native::c_ffi::StaticStrAsMinifiCStr;

            unsafe {
                let mut processor_list = minifi_native::c_ffi::CffiProcessorList::new();

                $(
                    {
                        processor_list.add::<$proc>();
                    }
                )*

                let mut controller_list = minifi_native::c_ffi::CffiControllerServiceList::new();

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

                minifi_native::sys::MinifiCreateExtension(&extension_create_info)
            }
        }
    };
}
