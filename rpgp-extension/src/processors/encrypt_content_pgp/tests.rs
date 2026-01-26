use super::*;
use minifi_native::{MockLogger, MockProcessContext};

#[test]
fn schedule_succeeds_with_default_values() {
    let mut processor = EncryptContentPGP::new(MockLogger::new());
    let context = MockProcessContext::new();

    assert_eq!(processor.on_schedule(&context), Ok(()));
}
