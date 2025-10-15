use minifi_native::{MockLogger, MockProcessContext, MockProcessSession};
use minifi_native::MinifiError::UnknownError;
use crate::processors::kamikaze_processor::properties::{ON_SCHEDULE_BEHAVIOUR, ON_TRIGGER_BEHAVIOUR};
use super::*;
use std::panic::{AssertUnwindSafe};

#[test]
fn on_schedule_ok() {
    let mut processor = KamikazeProcessor::new(MockLogger::new());
    let context = MockProcessContext::new();
    assert_eq!(processor.on_schedule(&context), Ok(()));
}

#[test]
fn on_schedule_err() {
    let mut processor = KamikazeProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(ON_SCHEDULE_BEHAVIOUR.name.to_string(), "ReturnErr".to_string());
    assert_eq!(processor.on_schedule(&context), Err(UnknownError));
}

#[test]
fn on_schedule_panic() {
    let mut processor = KamikazeProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(ON_SCHEDULE_BEHAVIOUR.name.to_string(), "Panic".to_string());

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| processor.on_schedule(&context)));
    assert!(result.is_err());
}

#[test]
fn on_trigger_ok() {
    let mut processor = KamikazeProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    assert_eq!(processor.on_schedule(&context), Ok(()));

    let mut session = MockProcessSession::new();
    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(()));
}

#[test]
fn on_trigger_err() {
    let mut processor = KamikazeProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(ON_TRIGGER_BEHAVIOUR.name.to_string(), "ReturnErr".to_string());
    assert_eq!(processor.on_schedule(&context), Ok(()));

    let mut session = MockProcessSession::new();
    assert_eq!(processor.on_trigger(&mut context, &mut session), Err(UnknownError));
}

#[test]
fn on_trigger_panic() {
    let mut processor = KamikazeProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(ON_TRIGGER_BEHAVIOUR.name.to_string(), "Panic".to_string());
    assert_eq!(processor.on_schedule(&context), Ok(()));

    let mut session = MockProcessSession::new();
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| processor.on_trigger(&mut context, &mut session)));
    assert!(result.is_err());
}
