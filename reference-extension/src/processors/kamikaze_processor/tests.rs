use minifi_native::{MockLogger, MockProcessContext};
use super::*;

#[test]
fn schedule_succeeds_with_default_values() {
    let mut processor = KamikazeProcessor::new(MockLogger::new());
    let context = MockProcessContext::new();

    assert_eq!(processor.on_schedule(&context), Ok(()));
}
