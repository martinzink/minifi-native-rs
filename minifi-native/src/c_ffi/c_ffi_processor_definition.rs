use std::ffi::c_void;
use std::ptr;

use super::c_ffi_logger::CffiLogger;
use super::c_ffi_primitives::{StaticStrAsMinifiCStr, StringView};
use super::c_ffi_process_context::CffiProcessContext;
use super::c_ffi_process_session::CffiProcessSession;
use crate::api::{Processor, ProcessorInputRequirement, ThreadingModel};
use crate::c_ffi::c_ffi_property::CProperties;
use crate::{Concurrent, ConcurrentOnTrigger, Exclusive, ExclusiveOnTrigger, LogLevel, OutputAttribute, Property};
use crate::{OnTriggerResult, Relationship};
use minifi_native_sys::*;
use crate::c_ffi::c_ffi_output_attribute::COutputAttributes;

pub trait DispatchOnTrigger<M: ThreadingModel> {
    unsafe fn dispatch_on_trigger(
        processor: *mut c_void,
        context: *mut MinifiProcessContext,
        session: *mut MinifiProcessSession,
    ) -> MinifiStatus;
}

impl<T> DispatchOnTrigger<Concurrent> for T
where
    T: ConcurrentOnTrigger<CffiLogger>,
{
    unsafe fn dispatch_on_trigger(
        processor_ptr: *mut c_void,
        context_ptr: *mut MinifiProcessContext,
        session_ptr: *mut MinifiProcessSession,
    ) -> MinifiStatus {
        unsafe {
            let processor = &*(processor_ptr as *const T);
            let mut context = CffiProcessContext::new(context_ptr);
            let mut session = CffiProcessSession::new(session_ptr);
            match processor.on_trigger(&mut context, &mut session) {
                Ok(OnTriggerResult::Ok) => MinifiStatus_MINIFI_STATUS_SUCCESS,
                Ok(OnTriggerResult::Yield) => MinifiStatus_MINIFI_STATUS_PROCESSOR_YIELD,
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
        context_ptr: *mut MinifiProcessContext,
        session_ptr: *mut MinifiProcessSession,
    ) -> MinifiStatus {
        unsafe {
            let processor = &mut *(processor_ptr as *mut T);
            let mut context = CffiProcessContext::new(context_ptr);
            let mut session = CffiProcessSession::new(session_ptr);
            match processor.on_trigger(&mut context, &mut session) {
                Ok(OnTriggerResult::Ok) => MinifiStatus_MINIFI_STATUS_SUCCESS,
                Ok(OnTriggerResult::Yield) => MinifiStatus_MINIFI_STATUS_PROCESSOR_YIELD,
                Err(error_code) => error_code.to_status(),
            }
        }
    }
}

pub struct ProcessorDefinition<T>
where
    T: Processor<CffiLogger> + DispatchOnTrigger<T::Threading>,
{
    name: &'static str,
    description_text: &'static str,
    input_requirement: ProcessorInputRequirement,
    supports_dynamic_properties: bool,
    supports_dynamic_relationships: bool,

    c_output_attributes: COutputAttributes,
    c_relationships: Vec<MinifiRelationshipDefinition>,
    c_properties: CProperties,

    _phantom: std::marker::PhantomData<T>,
}

impl<T> ProcessorDefinition<T>
where
    T: Processor<CffiLogger> + DispatchOnTrigger<T::Threading>,
{
    pub fn new(
        name: &'static str,
        description_text: &'static str,
        input_requirement: ProcessorInputRequirement,
        supports_dynamic_properties: bool,
        supports_dynamic_relationships: bool,
        output_attributes: &'static [OutputAttribute],
        relationships: &'static [Relationship],
        properties: &'static [Property],
    ) -> Self {
        let c_relationships = Relationship::create_c_vec(relationships);
        let c_properties = Property::create_c_properties(properties);
        let c_output_attributes= COutputAttributes::new(output_attributes);

        Self {
            name,
            description_text,
            input_requirement,
            supports_dynamic_properties,
            supports_dynamic_relationships,
            c_output_attributes,
            c_relationships,
            c_properties,
            _phantom: std::marker::PhantomData,
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
        context_ptr: *mut MinifiProcessContext,
        session_ptr: *mut MinifiProcessSession,
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
        context_ptr: *mut MinifiProcessContext,
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

    unsafe extern "C" fn is_work_available(processor_ptr: *mut c_void) -> bool {
        unsafe {
            let processor = &*(processor_ptr as *const T);
            processor.is_work_available()
        }
    }

    unsafe extern "C" fn restore(_processor_ptr: *mut c_void, _flow_file: *mut MinifiFlowFile) {
        eprintln!("Restore is not implemented for this processor.");
    }

    unsafe extern "C" fn get_trigger_when_empty(processor_ptr: *mut c_void) -> bool {
        unsafe {
            let processor = &*(processor_ptr as *const T);
            processor.get_trigger_when_empty()
        }
    }

    unsafe extern "C" fn calculate_metrics(
        processor_ptr: *mut c_void,
    ) -> *mut MinifiPublishedMetrics {
        unsafe {
            let processor = &*(processor_ptr as *const T);
            let metrics = processor.calculate_metrics();
            let metric_values: Vec<f64> = metrics.iter().map(|(_k, v)| *v).collect();
            // TODO(mzink) maybe we should skip StringView and use MinifiStringView directly?
            let metric_string_view: Vec<StringView> = metrics
                .iter()
                .map(|(k, _v)| StringView::new(k.as_str()))
                .collect();
            let metric_minifi_string_view: Vec<MinifiStringView> =
                metric_string_view.iter().map(|sv| sv.as_raw()).collect();
            assert_eq!(metric_values.len(), metric_minifi_string_view.len());
            assert_eq!(metrics.len(), metric_minifi_string_view.len());

            MinifiPublishedMetricsCreate(
                metrics.len(),
                metric_minifi_string_view.as_ptr(),
                metric_values.as_ptr(),
            )
        }
    }
}

pub trait DynProcessorDefinition {
    // unsafe because self must outlive the resulting MinifiProcessorClassDescription
    unsafe fn class_description(&self) -> MinifiProcessorClassDescription;
}

impl<T> DynProcessorDefinition for ProcessorDefinition<T>
where
    T: Processor<CffiLogger> + DispatchOnTrigger<T::Threading> {
    // unsafe because self must outlive the resulting MinifiProcessorClassDescription
    unsafe fn class_description(&self) -> MinifiProcessorClassDescription {
        unsafe {
            MinifiProcessorClassDescription {
                full_name: self.name.as_minifi_c_type(),
                description: self.description_text.as_minifi_c_type(),
                class_properties_count: self.c_properties.len(),
                class_properties_ptr: self.c_properties.get_ptr(),
                dynamic_properties_count: 0,
                dynamic_properties_ptr: ptr::null(),
                class_relationships_count: self.c_relationships.len(),
                class_relationships_ptr: self.c_relationships.as_ptr(),
                output_attributes_count: self.c_output_attributes.len(),
                output_attributes_ptr: self.c_output_attributes.get_ptr(),
                supports_dynamic_properties: self.supports_dynamic_properties,
                supports_dynamic_relationships: self.supports_dynamic_relationships,
                input_requirement: self.input_requirement.as_minifi_c_type(),
                is_single_threaded: T::Threading::IS_EXCLUSIVE,
                callbacks: MinifiProcessorCallbacks {
                    create: Some(Self::create_processor),
                    destroy: Some(Self::destroy_processor),
                    isWorkAvailable: Some(Self::is_work_available),
                    restore: Some(Self::restore),
                    getTriggerWhenEmpty: Some(Self::get_trigger_when_empty),
                    onTrigger: Some(Self::on_trigger_processor),
                    onSchedule: Some(Self::on_schedule_processor),
                    onUnSchedule: Some(Self::on_unschedule_processor),
                    calculateMetrics: Some(Self::calculate_metrics),
                },
            }
        }
    }
}