use crate::processors::dummy_processor::DummyProcessor;
use minifi_native::MinifiError::MissingRequiredProperty;
use minifi_native::{MockLogger, MockProcessContext, RawProcessor};

#[test]
fn on_schedule_fails_without_controller_service() {
    let mut processor = DummyProcessor::new(MockLogger::new());
    let context = MockProcessContext::new();

    assert_eq!(
        processor.on_schedule(&context),
        Err(MissingRequiredProperty("Dummy Controller Service"))
    );
}

#[test]
fn on_schedule_succeeds_with_any_controller_service() {
    let mut processor = DummyProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(
        "Dummy Controller Service".to_string(),
        "nonexistent_controller_service".to_string(),
    );

    assert_eq!(processor.on_schedule(&context), Ok(()));
}
