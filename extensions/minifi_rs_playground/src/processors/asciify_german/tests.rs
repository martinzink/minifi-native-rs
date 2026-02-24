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
        let result = asciify_german.transform(
            &mut context,
            &ff,
            &mut input_stream,
            &mut output_vec,
            &logger,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().target_relationship_name(), SUCCESS.name);
    }
    assert_eq!(
        output_vec,
        "Falsches Ueben von Xylophonmusik quaelt jeden groesseren Zwerg.".as_bytes()
    );
}
