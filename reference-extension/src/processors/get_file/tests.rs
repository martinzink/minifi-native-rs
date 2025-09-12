use std::path::Path;
use super::*;
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

fn create_test_directory() -> TempDir {
    fn make_file(temp_dir: &TempDir, file_name: &str, size: usize)  {
        let path = temp_dir.path().join(file_name);
        std::fs::write(&path, "a".repeat(size)).unwrap();
    }

    fn make_file_old(path: &Path) {
        filetime::set_file_mtime(
            path,
            FileTime::from_system_time(SystemTime::now() - humantime::parse_duration("1 day").unwrap()),
        )
            .unwrap();
    }

    let temp_dir = tempfile::tempdir().expect("temp dir is required for testing GetFile");
    let old_small_path = temp_dir.path().join("old_small.txt");
    let new_small_path = temp_dir.path().join("new_small.txt");
    std::fs::write(&old_small_path, "test").unwrap();
    std::fs::write(&new_small_path, "test").unwrap();

    temp_dir
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
