use super::DummyControllerService;
use super::properties::*;
use minifi_native::{
    ControllerServiceDefinition, RegisterableControllerService,
};

impl RegisterableControllerService for DummyControllerService {
    fn get_definition() -> Box<dyn minifi_native::DynControllerServiceDefinition> {
        Box::new(ControllerServiceDefinition::<DummyControllerService>::new(
            "Simple Rusty Controller Service to test API",
            &[DATA],
        ))
    }
}
