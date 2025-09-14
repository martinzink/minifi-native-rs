use super::*;
use crate::processors::get_file::relationships::SUCCESS;
use filetime::FileTime;
use minifi_native::{MockLogger, MockProcessContext, MockProcessSession};
use tempfile::TempDir;

#[test]
fn schedule_fails_without_valid_directory() {
    let mut processor = GetFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();

    assert_eq!(
        processor.on_schedule(&context),
        Err(MinifiError::MissingRequiredProperty("Input Directory"))
    );
    context.properties.insert(
        "Input Directory".to_string(),
        "/invalid_directory".to_string(),
    );
    assert_eq!(
        processor.on_schedule(&context),
        Err(MinifiError::ScheduleError(
            "\"/invalid_directory\" is not a valid directory".to_string()
        ))
    );
}

#[test]
fn simple_get_file_test() {
    let temp_dir = tempfile::tempdir().expect("temp dir is required for testing GetFile");
    let file_path = temp_dir.path().join("input_file");
    std::fs::write(&file_path, "test").unwrap();

    let mut processor = GetFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(
        "Input Directory".to_string(),
        temp_dir.path().to_str().unwrap().to_string(),
    );

    let mut session = MockProcessSession::new();
    assert!(processor.on_schedule(&context).is_ok());
    assert!(processor.on_trigger(&mut context, &mut session).is_ok());
    assert_eq!(session.transferred_flow_files.len(), 1);
}

fn create_test_directory() -> TempDir {
    fn make_file(temp_dir: &TempDir, file_name: &str, size: usize, age: Duration) {
        let path = temp_dir.path().join(file_name);
        std::fs::write(&path, "a".repeat(size)).unwrap();
        let file_time = FileTime::from_system_time(SystemTime::now() - age);
        filetime::set_file_mtime(path, file_time).expect("Cannot set file time");
    }

    let temp_dir = tempfile::tempdir().expect("temp dir is required for testing GetFile");
    make_file(&temp_dir, "small_new", 10, Duration::from_secs(10));
    make_file(&temp_dir, "small_old", 11, Duration::from_secs(3600));

    make_file(&temp_dir, "large_new", 1000, Duration::from_secs(0));
    make_file(&temp_dir, "large_old", 2000, Duration::from_secs(3600));
    temp_dir
}

#[test]
fn complex_dir_without_filters() {
    let test_directory = create_test_directory();

    let mut processor = GetFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(
        "Input Directory".to_string(),
        test_directory.path().to_str().unwrap().to_string(),
    );
    context
        .properties
        .insert("Batch Size".to_string(), "10".to_string());

    let mut session = MockProcessSession::new();
    assert!(processor.on_schedule(&context).is_ok());
    assert!(processor.on_trigger(&mut context, &mut session).is_ok());
    assert_eq!(session.transferred_flow_files.len(), 4);
}

fn test_complex_dir_with_filter(property_name: &str, property_vale: &str, expected_filename_part: &str) {
    let test_directory = create_test_directory();

    let mut processor = GetFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(
        DIRECTORY.name.to_string(),
        test_directory.path().to_str().unwrap().to_string(),
    );
    context
        .properties
        .insert(BATCH_SIZE.name.to_string(), "10".to_string());

    context
        .properties
        .insert(property_name.to_string(), property_vale.to_string());

    let mut session = MockProcessSession::new();
    assert!(processor.on_schedule(&context).is_ok());
    assert!(processor.on_trigger(&mut context, &mut session).is_ok());
    assert_eq!(session.transferred_flow_files.len(), 2);
    assert!(session.transferred_flow_files.iter().all(|transfer| {
        transfer.relationship == SUCCESS.name
            && transfer
            .flow_file
            .attributes
            .get("filename")
            .and_then(|filename| Some(filename.contains(expected_filename_part)))
            .unwrap_or(false)
    }))
}

#[test]
fn complex_dir_with_filters() {
    test_complex_dir_with_filter(MIN_AGE.name, "5 min", "old");
    test_complex_dir_with_filter(MAX_AGE.name, "5 min", "new");
    test_complex_dir_with_filter(MIN_SIZE.name, "50 B", "large");
    test_complex_dir_with_filter(MAX_SIZE.name, "50 B", "small");
}
