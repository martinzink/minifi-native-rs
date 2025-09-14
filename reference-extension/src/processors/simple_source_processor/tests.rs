use super::*;
use minifi_native::{MockLogger, MockProcessContext, MockProcessSession};
use crate::processors::simple_source_processor::relationships::SUCCESS;

#[test]
fn simple_test() {
    let mut processor = SimpleSourceProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context
        .properties
        .insert("Content".to_string(), "Hello, World!".to_string());
    processor.on_schedule(&context).expect("The on_schedule should succeed");

    {
        let mut session = MockProcessSession::new();
        processor.on_trigger(&mut context, &mut session).expect("The on_trigger should succeed");
        assert_eq!(session.transferred_flow_files.len(), 1);
        assert_eq!(session.transferred_flow_files[0].relationship, SUCCESS.name);
        assert_eq!(session.transferred_flow_files[0].flow_file.attributes.get("source").unwrap(), "SimpleSourceProcessor");
    }
}
