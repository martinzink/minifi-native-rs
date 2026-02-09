use crate::processors::dummy_processor::DummyProcessor;
use minifi_native::{MockLogger, MockProcessContext, Schedule};

#[test]
fn schedules_with_controller() {
    let context = MockProcessContext::new();
    let schedule_result = DummyProcessor::schedule(&context, &MockLogger::new());
    assert!(schedule_result.is_ok());
}
