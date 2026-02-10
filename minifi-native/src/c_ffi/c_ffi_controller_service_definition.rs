use crate::api::ControllerService;
use crate::c_ffi::c_ffi_controller_service_context::CffiControllerServiceContext;
use crate::c_ffi::c_ffi_property::CProperties;
use crate::{ComponentIdentifier, ControllerServiceDefinition, LogLevel, Property, StaticStrAsMinifiCStr};
use minifi_native_sys::{
    MinifiControllerServiceCallbacks, MinifiControllerServiceClassDefinition,
    MinifiControllerServiceContext, MinifiControllerServiceMetadata, MinifiStatus,
};
use std::ffi::c_void;

pub struct CffiControllerServiceDefinition<T>
where
    T: ControllerService + ComponentIdentifier,
{
    name: &'static str,
    description_text: &'static str,

    c_properties: CProperties,

    _phantom: std::marker::PhantomData<T>,
}

impl<T> CffiControllerServiceDefinition<T>
where
    T: ControllerService + ComponentIdentifier,
{
    pub fn new(
        description_text: &'static str,
        properties: &'static [Property],
    ) -> Self {
        let c_properties = Property::create_c_properties(properties);

        Self {
            name: T::CLASS_NAME,
            description_text,
            c_properties,
            _phantom: std::marker::PhantomData,
        }
    }

    #[cfg(not(feature = "mock-logger"))]
    unsafe extern "C" fn create_controller_service(
        metadata: MinifiControllerServiceMetadata,
    ) -> *mut c_void {
        let logger = super::c_ffi_logger::CffiLogger::new(metadata.logger);
        let controller_service = Box::new(T::new(logger));
        Box::into_raw(controller_service) as *mut c_void
    }

    #[cfg(feature = "mock-logger")]
    unsafe extern "C" fn create_controller_service(
        _metadata: MinifiControllerServiceMetadata,
    ) -> *mut c_void {
        panic!("mock-logger feature is on we should not create c controller services")
    }

    unsafe extern "C" fn destroy_controller_service(controller_service_ptr: *mut c_void) {
        unsafe {
            if !controller_service_ptr.is_null() {
                let _ = Box::from_raw(controller_service_ptr as *mut T);
            }
        }
    }

    unsafe extern "C" fn enable_controller_service(
        controller_service_ptr: *mut c_void,
        context_ptr: *mut MinifiControllerServiceContext,
    ) -> MinifiStatus {
        unsafe {
            let controller_service = &mut *(controller_service_ptr as *mut T);
            let context = CffiControllerServiceContext::new(context_ptr);
            match controller_service.enable(&context) {
                Ok(_) => 0,
                Err(err) => {
                    controller_service.log(LogLevel::Error, format!("{:?}", err).as_str());
                    err.to_status()
                }
            }
        }
    }

    unsafe extern "C" fn disable_controller_service(controller_service_ptr: *mut c_void) {
        unsafe {
            let controller_service = &mut *(controller_service_ptr as *mut T);
            controller_service.disable()
        }
    }
}

pub trait DynRawControllerServiceDefinition {
    // unsafe because self must outlive the resulting MinifiControllerServiceClassDefinition
    unsafe fn class_description(&self) -> MinifiControllerServiceClassDefinition;
}

impl<T> DynRawControllerServiceDefinition for CffiControllerServiceDefinition<T>
where
    T: ControllerService + ComponentIdentifier,
{
    unsafe fn class_description(&self) -> MinifiControllerServiceClassDefinition {
        unsafe {
            MinifiControllerServiceClassDefinition {
                full_name: self.name.as_minifi_c_type(),
                description: self.description_text.as_minifi_c_type(),
                class_properties_count: self.c_properties.len(),
                class_properties_ptr: self.c_properties.get_ptr(),
                callbacks: MinifiControllerServiceCallbacks {
                    create: Some(Self::create_controller_service),
                    destroy: Some(Self::destroy_controller_service),
                    enable: Some(Self::enable_controller_service),
                    notifyStop: Some(Self::disable_controller_service),
                },
            }
        }
    }
}

pub trait RegisterableControllerService {
    fn get_definition() -> Box<dyn DynRawControllerServiceDefinition>;
}

impl<T> RegisterableControllerService for T where T: ComponentIdentifier + ControllerServiceDefinition + ControllerService + 'static {
    fn get_definition() -> Box<dyn DynRawControllerServiceDefinition> {
        Box::new(CffiControllerServiceDefinition::<T>::new(
            T::DESCRIPTION,
            T::PROPERTIES,
        ))
    }
}
