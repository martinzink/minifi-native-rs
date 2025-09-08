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
        );
        assert!(processor.logger.logs.contains(&(LogLevel::Info, "batch [73, 110, 112]".to_string())));
        assert!(processor.logger.logs.contains(&(LogLevel::Info, "batch [117, 116, 32]".to_string())));
        assert!(processor.logger.logs.contains(&(LogLevel::Info, "batch [102, 102]".to_string())));
    }
}
