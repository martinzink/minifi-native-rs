use super::*;
use minifi_native::{ControllerService, MockControllerServiceContext, MockFlowFile, MockLogger, MockProcessContext, MockProcessSession};
use crate::test_utils;

#[test]
fn schedules_but_fails_to_encrypt_with_defaults() {
    let mut processor = EncryptContentPGP::new(MockLogger::new());
    let mut context = MockProcessContext::new();

    assert_eq!(processor.on_schedule(&context), Ok(()));
    let mut session = MockProcessSession::new();
    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(OnTriggerResult::Yield));

    let mut input_ff = MockFlowFile::new();
    input_ff.content = "foo".as_bytes().to_vec();
    session.input_flow_files.push(input_ff);

    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(OnTriggerResult::Ok));
    assert!(session.input_flow_files.is_empty());
    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].relationship, FAILURE.name);
    assert_eq!(session.transferred_flow_files[0].flow_file.content, "foo".as_bytes().to_vec());
}

#[test]
fn encrypts_via_passphrase() {
    let mut processor = EncryptContentPGP::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context.properties.insert("Passphrase".to_string(), "password".to_string());

    assert_eq!(processor.on_schedule(&context), Ok(()));
    let mut session = MockProcessSession::new();

    let mut input_ff = MockFlowFile::new();
    input_ff.content = "foo".as_bytes().to_vec();
    session.input_flow_files.push(input_ff);

    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(OnTriggerResult::Ok));
    assert!(session.input_flow_files.is_empty());
    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].relationship, SUCCESS.name);
    assert_ne!(session.transferred_flow_files[0].flow_file.content, "foo".as_bytes().to_vec());
}

fn public_key_service() -> PgpPublicKeyService {
    let mut controller_service = PgpPublicKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context
        .properties
        .insert("Keyring File".to_string(), test_utils::get_test_key_path("keyring.asc"));
    assert_eq!(controller_service.enable(&context), Ok(()));
    controller_service
}

#[test]
fn encrypts_via_public_key() {
    let mut processor = EncryptContentPGP::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    //context.controller_services.insert("my_public_key_service".to_string(), public_key_service());
    context.properties.insert("Public Key Service".to_string(), "password".to_string());

    assert_eq!(processor.on_schedule(&context), Ok(()));
    let mut session = MockProcessSession::new();

    let mut input_ff = MockFlowFile::new();
    input_ff.content = "foo".as_bytes().to_vec();
    session.input_flow_files.push(input_ff);

    assert_eq!(processor.on_trigger(&mut context, &mut session), Ok(OnTriggerResult::Ok));
    assert!(session.input_flow_files.is_empty());
    assert_eq!(session.transferred_flow_files.len(), 1);
    assert_eq!(session.transferred_flow_files[0].relationship, SUCCESS.name);
    assert_ne!(session.transferred_flow_files[0].flow_file.content, "foo".as_bytes().to_vec());
}
