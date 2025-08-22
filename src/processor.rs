pub use crate::wrapper::{Descriptor, Logger, ProcessContext, Session, SessionFactory};
use minifi_native_sys::*;
use std::ffi::c_void;
use std::ptr;
use crate::primitives::static_minifi_string_view;

/// A safe, idiomatic Rust trait for implementing a MiNiFi Processor.
pub trait Processor: Sized + 'static {
    fn new(logger: Logger) -> Self;

    fn initialize(&mut self, descriptor: &mut Descriptor);
    fn on_trigger(&mut self, context: &ProcessContext, session: &mut Session);
    fn on_schedule(&mut self, context: &ProcessContext, session_factory: &mut SessionFactory);

    fn get_name(&self) -> &'static str;
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
                module_name: static_minifi_string_view(module_name),
                short_name: static_minifi_string_view(short_name),
                full_name: static_minifi_string_view(full_name),
                description: static_minifi_string_view(description_text),
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
                    getProcessorType: Some(Self::get_processor_type),
                    getTriggerWhenEmpty: Some(Self::get_trigger_when_empty),
                    onSchedule: Some(Self::on_schedule_processor),
                    onUnSchedule: None,
                    notifyStop: None,
                    getInputRequirement: Some(Self::get_input_requirement),
                    serializeMetrics: None,
                    calculateMetrics: None,
                    forEachLogger: Some(Self::for_each_logger),
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
    ) -> MinifiStatus {
        let processor = &mut *(processor_ptr as *mut T);
        let context = ProcessContext::new(context_ptr);
        let mut session = Session::new(session_ptr);
        processor.on_trigger(&context, &mut session);
        0
    }

    unsafe extern "C" fn on_schedule_processor(
        processor_ptr: *mut c_void,
        context_ptr: MinifiProcessContext,
        session_factory_ptr: MinifiProcessSessionFactory,
    ) -> MinifiStatus {
        let processor = &mut *(processor_ptr as *mut T);
        let context = ProcessContext::new(context_ptr);
        let mut session_factory = SessionFactory::new(session_factory_ptr);
        processor.on_schedule(&context, &mut session_factory);
        0
    }

    unsafe extern "C" fn initialize_processor(
        processor_ptr: *mut c_void,
        descriptor_ptr: MinifiProcessorDescriptor,
    ) {
        let processor = &mut *(processor_ptr as *mut T);
        let mut descriptor = Descriptor::new(descriptor_ptr);
        processor.initialize(&mut descriptor);
    }

    unsafe extern "C" fn for_each_logger(
        _processor_ptr: *mut c_void,
        _minifi_logger_callback: MinifiLoggerCallback
    ) {
        // TODO(mzink): Implement this
    }

    unsafe extern "C" fn get_input_requirement(
        processor_ptr: *mut c_void,
    ) -> MinifiInputRequirement {
        MinifiInputRequirement_MINIFI_INPUT_ALLOWED
    }

    unsafe extern "C" fn get_processor_type(
        processor_ptr: *mut c_void,
    ) -> MinifiString {
        let processor = &mut *(processor_ptr as *mut T);
        let minifi_string_view = MinifiStringView {
            data: processor.get_name().as_ptr() as *const i8,
            length: processor.get_name().len() as u32,
        };
        MinifiCreateString(minifi_string_view)
    }

    unsafe extern "C" fn get_trigger_when_empty(
        processor_ptr: *mut c_void,
    ) -> MinifiBool {
        MINIFI_FALSE
    }
}
