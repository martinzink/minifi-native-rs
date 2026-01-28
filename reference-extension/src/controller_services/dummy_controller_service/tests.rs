use minifi_native::{ControllerService, MockControllerServiceContext, MockLogger};
use minifi_native::MinifiError::{MissingRequiredProperty};
use crate::controller_services::dummy_controller_service::DummyControllerService;

#[test]
fn enable_fails_without_data_property() {
    let mut controller_service = DummyControllerService::new(MockLogger::new());
    let context = MockControllerServiceContext::new();


    assert_eq!(controller_service.enable(&context), Err(MissingRequiredProperty("Data")));
}


#[test]
fn enable_succeeds_with_data_property() {
    let mut controller_service = DummyControllerService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context.properties.insert("Data".to_string(), "foo".to_string());


    assert_eq!(controller_service.enable(&context), Ok(()));
    assert_eq!(controller_service.get_data(), Some("foo"));
}
