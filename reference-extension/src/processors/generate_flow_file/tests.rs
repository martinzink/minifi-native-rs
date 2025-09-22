use super::*;
use minifi_native::{MockLogger, MockProcessContext, MockProcessSession};
use crate::processors::generate_flow_file::properties::{DATA_FORMAT, UNIQUE_FLOW_FILES};

#[test]
fn generate_flow_file_empty_test() {
    let mut processor = GenerateFlowFile::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert(
        properties::FILE_SIZE.name.to_string(),
        "0".to_string(),
    );
    context.properties.insert(
        UNIQUE_FLOW_FILES.name.to_string(),
        "false".to_string(),
    );
    context.properties.insert(
        DATA_FORMAT.name.to_string(),
        "Text".to_string(),
    );

    assert!(processor.on_schedule(&context).is_ok());
    let mut session = MockProcessSession::new();
    assert!(processor.on_trigger(&mut context, &mut session).is_ok());
    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].flow_file.content.len(), 0);
}
