use super::*;
use minifi_native::{MockLogger, MockProcessContext, MockProcessSession};

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
        let created_flow_file = session
            .transferred_flow_files
            .get(relationships::SUCCESS.name)
            .unwrap();
        assert_eq!(created_flow_file.content, "Hello, World!");
        assert_eq!(
            created_flow_file.attributes.get("source").unwrap(),
            "SimpleSourceProcessor"
        );
    }
}
