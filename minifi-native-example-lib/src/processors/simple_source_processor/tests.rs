use super::*;
use minifi_native::{MockLogger, MockProcessContext, MockProcessSession};

#[test]
fn simple_test() {
    let mut processor = SimpleSourceProcessor::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context
        .properties
        .insert("Content".to_string(), "Hello, World!".to_string());
    processor.on_schedule(&context);

    {
        let mut session = MockProcessSession::new();
        processor.on_trigger(&context, &mut session);
        let created_flow_file = session
            .transferred_flow_files
            .get(SUCCESS_RELATIONSHIP.name)
            .unwrap();
        assert_eq!(created_flow_file.content, "Hello, World!");
        assert_eq!(created_flow_file.attributes.get("source").unwrap(), "SimpleSourceProcessor");
    }
}
