use super::DummyControllerService;
use super::properties::*;
use minifi_native::{CffiControllerServiceDefinition, RegisterableControllerService};

impl RegisterableControllerService for DummyControllerService {
    fn get_definition() -> Box<dyn minifi_native::DynRawControllerServiceDefinition> {
        Box::new(
            CffiControllerServiceDefinition::<DummyControllerService>::new(
                "Simple Rusty Controller Service to test API",
                &[DATA],
            ),
        )
    }
}
