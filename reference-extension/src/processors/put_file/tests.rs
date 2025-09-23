use super::*;
use crate::processors::put_file::relationships::SUCCESS;
use minifi_native::{MockFlowFile, MockLogger, MockProcessContext, MockProcessSession};

#[test]
fn schedule_succeeds_with_default_values() {
    let mut processor = PutFile::new(MockLogger::new());
    let context = MockProcessContext::new();

    assert_eq!(processor.on_schedule(&context), Ok(()));
}

#[test]
fn simple_put_file_test() {
    let mut put_file = PutFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    let temp_dir = tempfile::tempdir().expect("temp dir is required for testing PutFile");

    context.properties.insert(
        "Directory".to_string(),
        temp_dir.path().to_str().unwrap().to_string(),
    );
    assert_eq!(put_file.on_schedule(&context), Ok(()));

    let mut session = MockProcessSession::new();
    let mut flow_file = MockFlowFile::new();
    flow_file
        .attributes
        .insert("filename".to_string(), "test.txt".to_string());
    flow_file.content = "test".as_bytes().to_vec();
    session.input_flow_files.push(flow_file);

    assert_eq!(put_file.on_trigger(&mut context, &mut session), Ok(()));

    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].relationship, SUCCESS.name);

    let expected_path = temp_dir.path().join("test.txt");
    assert!(expected_path.exists());
    assert_eq!(std::fs::read_to_string(expected_path).unwrap(), "test");
}
