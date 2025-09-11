use super::*;
use minifi_native::{MockLogger, MockProcessContext};

#[test]
fn simple_test() {
    let mut processor = GetFile::new(MockLogger::new());
    let context = MockProcessContext::new();

    processor
        .on_schedule(&context)
        .expect("The on_schedule should succeed");

}
