use super::*;
use minifi_native::{
    MockFlowFile, MockLogger, MockProcessContext, MockProcessSession,
};

#[test]
fn simple_test() {
    let mut processor = SimpleLogProcessor::new(MockLogger::new());
    let context = MockProcessContext::new();

    processor.on_schedule(&context);

    {
        let mut session = MockProcessSession::new();
        let mut input_ff = MockFlowFile::new();
        input_ff.content = "Input ff".to_string();
        session.input_flow_files.push(input_ff);
        processor.on_trigger(&context, &mut session);
        assert_eq!(
            session
                .transferred_flow_files
                .get("success")
                .unwrap()
                .content,
            "Input ff"
        )
    }
}
