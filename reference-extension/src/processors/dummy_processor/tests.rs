use minifi_native::{MockLogger, MockProcessContext, Schedule};
use crate::processors::dummy_processor::DummyProcessor;

#[test]
fn schedules_with_controller() {
    let context = MockProcessContext::new();
    let schedule_result = DummyProcessor::schedule(&context, &MockLogger::new());
    assert!(schedule_result.is_ok());
}