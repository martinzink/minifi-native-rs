use super::*;
use crate::processors::asciify_german::relationships::SUCCESS;
use minifi_native::{MockFlowFile, MockLogger, MockProcessContext};
use std::io::BufReader;

#[test]
fn schedule_succeeds_with_default_values() {
    assert!(AsciifyGerman::schedule(&MockProcessContext::new(), &MockLogger::new()).is_ok());
}

#[test]
fn simple_test() {
    let mut context = MockProcessContext::new();
    let ff = MockFlowFile::new();
    let logger = MockLogger::new();
    let asciify_german = AsciifyGerman::schedule(&context, &logger).expect("Should succeed");
    let input_str = "Falsches Üben von Xylophonmusik quält jeden größeren Zwerg.";
    let mut input_stream = BufReader::new(input_str.as_bytes());
    let mut output_vec: Vec<u8> = Vec::new();
    {
        let result = asciify_german
            .transform(
                &mut context,
                &ff,
                &mut input_stream,
                &mut output_vec,
                &logger,
            )
            .expect("Should succeed");
        // assert_eq!(result.modify_content(), StreamingOperationState::Ok); todo!
        assert_eq!(result.target_relationship_name(), SUCCESS.name);
    }
    assert_eq!(
        output_vec,
        "Falsches Ueben von Xylophonmusik quaelt jeden groesseren Zwerg.".as_bytes()
    );
}

#[test]
fn simple_failure_test() {
    let mut context = MockProcessContext::new();
    let ff = MockFlowFile::new();
    let logger = MockLogger::new();
    let asciify_german = AsciifyGerman::schedule(&context, &logger).expect("Should succeed");
    let input_str = "Üldögélő műújságíró";
    let mut input_stream = BufReader::new(input_str.as_bytes());
    let mut output_vec: Vec<u8> = Vec::new();
    {
        let result = asciify_german
            .transform(
                &mut context,
                &ff,
                &mut input_stream,
                &mut output_vec,
                &logger,
            )
            .expect("Should succeed");
        // assert!(!result.modify_content()); todo!
        assert_eq!(result.target_relationship_name(), FAILURE.name);
    }
    assert_eq!(output_vec, "Ueldoeg".as_bytes());
}
