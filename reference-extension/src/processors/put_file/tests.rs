use super::*;
use crate::processors::put_file::relationships::{FAILURE, SUCCESS};
use minifi_native::{MockFlowFile, MockLogger, MockProcessContext, MockProcessSession};

#[test]
fn schedule_succeeds_with_default_values() {
    assert!(PutFile::schedule(&MockProcessContext::new(), &MockLogger::new()).is_ok());

}

#[test]
fn simple_put_file_test() {
    let mut context = MockProcessContext::new();
    let temp_dir = tempfile::tempdir().expect("temp dir is required for testing PutFile");
    let put_file_dir = temp_dir.path().join("subdir");

    context.properties.insert(
        "Directory".to_string(),
        put_file_dir.to_str().unwrap().to_string(),
    );
    let mut put_file = PutFile::schedule(&context, &MockLogger::new()).expect("Should succeed");

    let mut session = MockProcessSession::new();
    let mut flow_file = MockFlowFile::new();
    flow_file
        .attributes
        .insert("filename".to_string(), "test.txt".to_string());
    flow_file.content = "test".as_bytes().to_vec();
    session.input_flow_files.push(flow_file);

    assert_eq!(
        put_file.trigger(&mut context, &mut session, &MockLogger::new()),
        Ok(OnTriggerResult::Ok)
    );

    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].relationship, SUCCESS.name);

    let expected_path = temp_dir.path().join("subdir/test.txt");
    assert!(expected_path.exists());
    assert_eq!(std::fs::read_to_string(expected_path).unwrap(), "test");
}

#[test]
fn put_file_without_create_dirs() {
    let mut context = MockProcessContext::new();
    let temp_dir = tempfile::tempdir().expect("temp dir is required for testing PutFile");

    let put_file_dir = temp_dir.path().join("subdir");

    context.properties.insert(
        "Directory".to_string(),
        put_file_dir.to_str().unwrap().to_string(),
    );

    context.properties.insert(
        "Create Missing Directories".to_string(),
        "false".to_string(),
    );

    let mut put_file = PutFile::schedule(&context, &MockLogger::new()).expect("Should succeed");

    let mut session = MockProcessSession::new();
    let mut flow_file = MockFlowFile::new();
    flow_file
        .attributes
        .insert("filename".to_string(), "test.txt".to_string());
    flow_file.content = "test".as_bytes().to_vec();
    session.input_flow_files.push(flow_file);

    assert_eq!(
        put_file.trigger(&mut context, &mut session, &MockLogger::new()),
        Ok(OnTriggerResult::Ok)
    );

    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].relationship, FAILURE.name);

    let expected_path = temp_dir.path().join("subdir/test.txt");
    assert!(!expected_path.exists());
}

#[cfg(unix)]
#[test]
fn put_file_test_permissions() {
    use std::os::unix::fs::PermissionsExt;
    let mut context = MockProcessContext::new();
    let temp_dir = tempfile::tempdir().expect("temp dir is required for testing PutFile");
    let put_file_dir = temp_dir.path().join("subdir");

    context.properties.insert(
        "Directory".to_string(),
        put_file_dir.to_str().unwrap().to_string(),
    );

    context
        .properties
        .insert("Directory Permissions".to_string(), "0777".to_string());

    context
        .properties
        .insert("Permissions".to_string(), "0777".to_string());
    let mut put_file = PutFile::schedule(&context, &MockLogger::new()).expect("Should succeed");

    let mut session = MockProcessSession::new();
    let mut flow_file = MockFlowFile::new();
    flow_file
        .attributes
        .insert("filename".to_string(), "test.txt".to_string());
    flow_file.content = "test".as_bytes().to_vec();
    session.input_flow_files.push(flow_file);

    assert_eq!(
        put_file.trigger(&mut context, &mut session, &MockLogger::new()),
        Ok(OnTriggerResult::Ok)
    );

    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].relationship, SUCCESS.name);

    let expected_path = temp_dir.path().join("subdir/test.txt");
    assert!(expected_path.exists());
    assert_eq!(std::fs::read_to_string(&expected_path).unwrap(), "test");
    let parent_permissions = std::fs::metadata(put_file_dir).unwrap().permissions();
    let permissions = expected_path.metadata().unwrap().permissions();
    assert_eq!(permissions.mode(), 0o100777);
    assert_eq!(parent_permissions.mode(), 0o40777);
}
