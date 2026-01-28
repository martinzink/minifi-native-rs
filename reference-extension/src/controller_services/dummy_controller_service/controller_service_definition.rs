use minifi_native::{ControllerService, ControllerServiceDefinition, RegisterableControllerService};
use super::properties::*;
use super::{DummyControllerService};

impl RegisterableControllerService for DummyControllerService {
    fn get_definition() -> Box<dyn minifi_native::DynControllerServiceDefinition> {
        Box::new(ControllerServiceDefinition::<DummyControllerService>::new(
            DummyControllerService::class_name(),
            "Simple Rusty Controller Service to test API",
            &[
                DATA
            ],
        ))
    }
}
