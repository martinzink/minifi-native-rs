use std::ffi::c_void;
use std::ptr;

use super::c_ffi_logger::CffiLogger;
use super::c_ffi_primitives::{BoolAsMinifiCBool, StaticStrAsMinifiCStr};
use super::c_ffi_process_context::CffiProcessContext;
use super::c_ffi_process_session::CffiProcessSession;
use crate::Relationship;
use crate::api::{Processor, ProcessorInputRequirement, ThreadingModel};
use crate::{Concurrent, ConcurrentOnTrigger, Exclusive, ExclusiveOnTrigger, LogLevel, Property};
use minifi_native_sys::*;

pub trait DispatchOnTrigger<M: ThreadingModel> {
    unsafe fn dispatch_on_trigger(
        processor: *mut c_void,
        context: MinifiProcessContext,
        session: MinifiProcessSession,
    ) -> MinifiStatus;
}

impl<T> DispatchOnTrigger<Concurrent> for T
where
    T: ConcurrentOnTrigger<CffiLogger>,
{
    unsafe fn dispatch_on_trigger(
        processor_ptr: *mut c_void,
        context_ptr: MinifiProcessContext,
        session_ptr: MinifiProcessSession,
    ) -> MinifiStatus {
        unsafe {
            let processor = &*(processor_ptr as *const T);
            let mut context = CffiProcessContext::new(context_ptr);
            let mut session = CffiProcessSession::new(session_ptr);
            match processor.on_trigger(&mut context, &mut session) {
                Ok(_) => 0,
                Err(error_code) => error_code.to_status(),
            }
        }
    }
}

impl<T> DispatchOnTrigger<Exclusive> for T
where
    T: ExclusiveOnTrigger<CffiLogger>,
{
    unsafe fn dispatch_on_trigger(
        processor_ptr: *mut c_void,
        context_ptr: MinifiProcessContext,
        session_ptr: MinifiProcessSession,
    ) -> MinifiStatus {
        unsafe {
            let processor = &mut *(processor_ptr as *mut T);
            let mut context = CffiProcessContext::new(context_ptr);
            let mut session = CffiProcessSession::new(session_ptr);
            match processor.on_trigger(&mut context, &mut session) {
                Ok(_) => 0,
                Err(error_code) => error_code.to_status(),
            }
        }
    }
}

pub struct ProcessorDefinition<T>
where
    T: Processor<CffiLogger> + DispatchOnTrigger<T::Threading>,
{
    module_name: &'static str,
    name: &'static str,
    description_text: &'static str,
    pub input_requirement: ProcessorInputRequirement,
    pub supports_dynamic_properties: bool,
    pub supports_dynamic_relationships: bool,
    pub is_single_threaded: bool,
    pub relationships: &'static [Relationship],
    pub properties: &'static [Property],
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ProcessorDefinition<T>
where
    T: Processor<CffiLogger> + DispatchOnTrigger<T::Threading>,
{
    pub fn new(
        module_name: &'static str,
        name: &'static str,
        description_text: &'static str,
    ) -> Self {
        Self {
            module_name,
            name,
            description_text,
            input_requirement: ProcessorInputRequirement::Allowed,
            supports_dynamic_properties: false,
            supports_dynamic_relationships: false,
            is_single_threaded: false,
            relationships: &[],
            properties: &[],
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn register_class(&self) {
        let c_relationships = Relationship::create_c_vec(&self.relationships);

        let c_default_values = Property::create_c_default_value_holder(&self.properties);
        let c_allowed_values = Property::create_c_allowed_values_vec_vec(&self.properties);
        let c_allowed_types = Property::create_c_allowed_types_vec_vec(&self.properties);
        let c_validators = Property::create_c_validators_vec(&self.properties);

        let c_properties = Property::create_c_properties(
            &self.properties,
            &c_default_values,
            &c_allowed_values,
            &c_allowed_types,
            &c_validators,
        );

        let class_description = MinifiProcessorClassDescription {
            module_name: self.module_name.as_minifi_c_type(),
            full_name: self.name.as_minifi_c_type(),
            description: self.description_text.as_minifi_c_type(),
            class_properties_count: c_properties.properties.len() as u32,
            class_properties_ptr: c_properties.properties.as_ptr(),
            dynamic_properties_count: 0,
            dynamic_properties_ptr: ptr::null(),
            class_relationships_count: c_relationships.len() as u32,
            class_relationships_ptr: c_relationships.as_ptr(),
            output_attributes_count: 0,
            output_attributes_ptr: ptr::null(),
            supports_dynamic_properties: self.supports_dynamic_properties.as_minifi_c_type(),
            supports_dynamic_relationships: self.supports_dynamic_relationships.as_minifi_c_type(),
            input_requirement: self.input_requirement.as_minifi_c_type(),
            is_single_threaded: self.is_single_threaded.as_minifi_c_type(),
            callbacks: MinifiProcessorCallbacks {
                create: Some(Self::create_processor),
                destroy: Some(Self::destroy_processor),
                isWorkAvailable: Some(Self::is_work_available),
                restore: Some(Self::restore),
                getTriggerWhenEmpty: Some(Self::get_trigger_when_empty),
                onTrigger: Some(Self::on_trigger_processor),
                onSchedule: Some(Self::on_schedule_processor),
                onUnSchedule: Some(Self::on_unschedule_processor),
                calculateMetrics: None,
            },
        };

        unsafe {
            MinifiRegisterProcessorClass(&class_description);
        }
    }

    // --- Unsafe FFI callback implementations ---

    unsafe extern "C" fn create_processor(metadata: MinifiProcessorMetadata) -> *mut c_void {
        let logger = CffiLogger::new(metadata.logger);
        let processor = Box::new(T::new(logger));
        Box::into_raw(processor) as *mut c_void
    }

    unsafe extern "C" fn destroy_processor(processor_ptr: *mut c_void) {
        unsafe {
            if !processor_ptr.is_null() {
                let _ = Box::from_raw(processor_ptr as *mut T);
            }
        }
    }

    unsafe extern "C" fn on_trigger_processor(
        processor_ptr: *mut c_void,
        context_ptr: MinifiProcessContext,
        session_ptr: MinifiProcessSession,
    ) -> MinifiStatus {
        unsafe {
            <T as DispatchOnTrigger<T::Threading>>::dispatch_on_trigger(
                processor_ptr,
                context_ptr,
                session_ptr,
            )
        }
    }

    unsafe extern "C" fn on_schedule_processor(
        processor_ptr: *mut c_void,
        context_ptr: MinifiProcessContext,
    ) -> MinifiStatus {
        unsafe {
            let processor = &mut *(processor_ptr as *mut T);
            let context = CffiProcessContext::new(context_ptr);
            match processor.on_schedule(&context) {
                Ok(_) => 0,
                Err(error_code) => {
                    processor.log(LogLevel::Error, format!("{:?}", error_code).as_str());
                    error_code.to_status()
                }
            }
        }
    }

    unsafe extern "C" fn on_unschedule_processor(processor_ptr: *mut c_void) {
        unsafe {
            let processor = &mut *(processor_ptr as *mut T);
            processor.on_unschedule();
        }
    }

    unsafe extern "C" fn is_work_available(processor_ptr: *mut c_void) -> MinifiBool {
        unsafe {
            let processor = &*(processor_ptr as *const T);
            processor.is_work_available().as_minifi_c_type()
        }
    }

    unsafe extern "C" fn restore(_processor_ptr: *mut c_void, _flow_file: *mut MinifiFlowFile_T) {
        eprintln!("Restore is not implemented for this processor.");
    }

    unsafe extern "C" fn get_trigger_when_empty(processor_ptr: *mut c_void) -> MinifiBool {
        unsafe {
            let processor = &*(processor_ptr as *const T);
            processor.get_trigger_when_empty().as_minifi_c_type()
        }
    }
}
