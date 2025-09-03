use minifi_native::{MockFlowFile, MockLogger, MockProcessContext, MockProcessSession, MockProcessSessionFactory};
use super::*;

#[test]
fn simple_test() {
    let mut processor = SimpleLogProcessor::new(MockLogger::new());
    let context = MockProcessContext::new();
    let mut session_factory = MockProcessSessionFactory{};

    processor.on_schedule(&context, &mut session_factory);

    {
        let mut session = MockProcessSession::new();
        let mut input_ff = MockFlowFile::new();
        input_ff.content = "Input ff".to_string();
        session.input_flow_files.push(input_ff);
        processor.on_trigger(&context, &mut session);
        assert_eq!(session.transferred_flow_files.get("success").unwrap().content, "Input ff")
    }
}