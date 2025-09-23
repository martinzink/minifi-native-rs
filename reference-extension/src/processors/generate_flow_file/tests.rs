use super::*;
use crate::processors::generate_flow_file::properties::{
    BATCH_SIZE, CUSTOM_TEXT, DATA_FORMAT, UNIQUE_FLOW_FILES,
};
use minifi_native::{MockLogger, MockProcessContext, MockProcessSession};

#[test]
fn schedule_succeeds_with_default_values() {
    let mut processor = GenerateFlowFile::new(MockLogger::new());
    let context = MockProcessContext::new();

    assert_eq!(processor.on_schedule(&context), Ok(()));
}

#[test]
fn generate_flow_file_empty_test() {
    let mut processor = GenerateFlowFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context
        .properties
        .insert(properties::FILE_SIZE.name.to_string(), "0".to_string());
    context
        .properties
        .insert(UNIQUE_FLOW_FILES.name.to_string(), "false".to_string());
    context
        .properties
        .insert(DATA_FORMAT.name.to_string(), "Text".to_string());

    assert_eq!(processor.on_schedule(&context), Ok(()));
    let mut session = MockProcessSession::new();
    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(()));
    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].flow_file.content.len(), 0);
}

#[test]
fn generate_custom_text() {
    let mut processor = GenerateFlowFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context
        .properties
        .insert(properties::FILE_SIZE.name.to_string(), "0".to_string());
    context
        .properties
        .insert(UNIQUE_FLOW_FILES.name.to_string(), "false".to_string());
    context
        .properties
        .insert(DATA_FORMAT.name.to_string(), "Text".to_string());
    context
        .properties
        .insert(CUSTOM_TEXT.name.to_string(), "foo bar baz".to_string());

    assert_eq!(processor.on_schedule(&context), Ok(()));
    let mut session = MockProcessSession::new();
    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(()));
    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(
        session.transferred_flow_files[0].flow_file.content,
        "foo bar baz".as_bytes()
    );
}

#[test]
fn random_bytes_unique() {
    let mut processor = GenerateFlowFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context
        .properties
        .insert(properties::FILE_SIZE.name.to_string(), "40 B".to_string());
    context
        .properties
        .insert(UNIQUE_FLOW_FILES.name.to_string(), "true".to_string());
    context
        .properties
        .insert(DATA_FORMAT.name.to_string(), "Bytes".to_string());
    context
        .properties
        .insert(BATCH_SIZE.name.to_string(), "2".to_string());

    assert_eq!(processor.on_schedule(&context), Ok(()));
    let mut session = MockProcessSession::new();
    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(()));
    assert_eq!(session.transferred_flow_files.len(), 2);
    assert_eq!(
        session.transferred_flow_files[0].flow_file.content.len(),
        40
    );
    assert_eq!(
        session.transferred_flow_files[1].flow_file.content.len(),
        40
    );
    assert_ne!(
        session.transferred_flow_files[0].flow_file.content,
        session.transferred_flow_files[1].flow_file.content
    );
}

#[test]
fn random_bytes_non_unique() {
    let mut processor = GenerateFlowFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context
        .properties
        .insert(properties::FILE_SIZE.name.to_string(), "40 B".to_string());
    context
        .properties
        .insert(UNIQUE_FLOW_FILES.name.to_string(), "false".to_string());
    context
        .properties
        .insert(DATA_FORMAT.name.to_string(), "Bytes".to_string());
    context
        .properties
        .insert(BATCH_SIZE.name.to_string(), "2".to_string());

    assert_eq!(processor.on_schedule(&context), Ok(()));
    let mut session = MockProcessSession::new();
    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(()));
    assert_eq!(session.transferred_flow_files.len(), 2);
    assert_eq!(
        session.transferred_flow_files[0].flow_file.content.len(),
        40
    );
    assert_eq!(
        session.transferred_flow_files[1].flow_file.content.len(),
        40
    );
    assert_eq!(
        session.transferred_flow_files[0].flow_file.content,
        session.transferred_flow_files[1].flow_file.content
    );
}
