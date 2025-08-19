// minifi/src/processor.rs

use crate::wrapper::{Descriptor, Logger, ProcessContext, Session};
use minifi_sys::*; // Import all the raw C types from the -sys crate
use std::ffi::c_void;
use std::ptr;

/// A safe, idiomatic Rust trait for implementing a MiNiFi Processor.
pub trait Processor: Sized + 'static {
    /// Called once to create an instance of your processor.
    fn new(logger: Logger) -> Self;

    /// Called to set the processor's supported properties and relationships.
    fn initialize(&mut self, descriptor: &mut Descriptor);

    /// The main entry point for your processor's logic.
    fn on_trigger(&mut self, context: &ProcessContext, session: &mut Session);
}

/// A generic FFI bridge that wraps any struct implementing the `Processor` trait.
/// This struct is public so it can be used in the final processor binary, but
/// is not intended for direct use by most developers.
pub struct ProcessorBridge<T: Processor> {
    pub description: MinifiProcessorClassDescription,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Processor> ProcessorBridge<T> {
    pub fn new(
        module_name: &'static str,
        short_name: &'static str,
        full_name: &'static str,
        description_text: &'static str,
    ) -> Self {
        Self {
            description: MinifiProcessorClassDescription {
                module_name: MinifiStringView {
                    data: module_name.as_ptr() as *const i8,
                    length: module_name.len() as u32,
                },
                short_name: MinifiStringView {
                    data: short_name.as_ptr() as *const i8,
                    length: short_name.len() as u32,
                },
                full_name: MinifiStringView {
                    data: full_name.as_ptr() as *const i8,
                    length: full_name.len() as u32,
                },
                description: MinifiStringView {
                    data: description_text.as_ptr() as *const i8,
                    length: description_text.len() as u32,
                },
                callbacks: MinifiProcessorCallbacks {
                    create: Some(Self::create_processor),
                    destroy: Some(Self::destroy_processor),
                    onTrigger: Some(Self::on_trigger_processor),
                    initialize: Some(Self::initialize_processor),
                    isWorkAvailable: None,
                    restore: None,
                    supportsDynamicProperties: None,
                    supportsDynamicRelationships: None,
                    isSingleThreaded: None,
                    getProcessorType: None,
                    getTriggerWhenEmpty: None,
                    onSchedule: None,
                    onUnSchedule: None,
                    notifyStop: None,
                    getInputRequirement: None,
                    serializeMetrics: None,
                    calculateMetrics: None,
                    forEachLogger: None,
                },
                class_properties_count: 0,
                class_properties_ptr: ptr::null(),
                dynamic_properties_count: 0,
                dynamic_properties_ptr: ptr::null(),
                class_relationships_count: 0,
                class_relationships_ptr: ptr::null(),
                output_attributes_count: 0,
                output_attributes_ptr: ptr::null(),
                supports_dynamic_properties: MINIFI_FALSE,
                supports_dynamic_relationships: MINIFI_FALSE,
                input_requirement: MinifiInputRequirement_MINIFI_INPUT_REQUIRED,
                is_single_threaded: MINIFI_FALSE,
                internal_name: MinifiStringView { data: ptr::null(), length: 0 },
            },
            _phantom: std::marker::PhantomData,
        }
    }

    // --- Unsafe FFI callback implementations ---

    unsafe extern "C" fn create_processor(metadata: MinifiProcessorMetadata) -> *mut c_void {
        let logger = Logger::new(metadata.logger);
        let processor = Box::new(T::new(logger));
        Box::into_raw(processor) as *mut c_void
    }

    unsafe extern "C" fn destroy_processor(processor_ptr: *mut c_void) {
        if !processor_ptr.is_null() {
            let _ = Box::from_raw(processor_ptr as *mut T);
        }
    }

    unsafe extern "C" fn on_trigger_processor(
        processor_ptr: *mut c_void,
        context_ptr: MinifiProcessContext,
        session_ptr: MinifiProcessSession,
        _error: MinifiString,
    ) {
        let processor = &mut *(processor_ptr as *mut T);
        let context = ProcessContext::new(context_ptr);
        let mut session = Session::new(session_ptr);
        processor.on_trigger(&context, &mut session);
    }

    unsafe extern "C" fn initialize_processor(
        processor_ptr: *mut c_void,
        descriptor_ptr: MinifiProcessorDescriptor,
    ) {
        let processor = &mut *(processor_ptr as *mut T);
        let mut descriptor = Descriptor::new(descriptor_ptr);
        processor.initialize(&mut descriptor);
    }
}
