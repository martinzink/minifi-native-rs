use super::*;
use crate::test_utils;
use minifi_native::{
    ComponentIdentifier, Content, EnableControllerService, MockControllerServiceContext,
    MockFlowFile, MockLogger, MockProcessContext, TransformedFlowFile,
};

#[test]
fn test_ids() {
    assert_eq!(
        EncryptContentPGP::CLASS_NAME,
        "minifi_pgp::processors::encrypt_content::EncryptContentPGP"
    );
    assert_eq!(EncryptContentPGP::GROUP_NAME, "minifi_pgp");
    assert_eq!(EncryptContentPGP::VERSION, "0.1.0");
}

#[test]
fn cannot_schedule_without_password_or_public_key() {
    assert!(EncryptContentPGP::schedule(&MockProcessContext::new(), &MockLogger::new()).is_err());
}

fn assert_content(transform_result: &TransformedFlowFile, is_ascii: bool) {
    assert_eq!(transform_result.target_relationship(), &SUCCESS);
    match transform_result.new_content() {
        Some(Content::Buffer(content)) => {
            assert_eq!(content.is_ascii(), is_ascii);
        }
        _ => {
            panic!("should be buffer content");
        }
    }
    assert_eq!(
        transform_result
            .attributes_to_add()
            .get("pgp.file.encoding")
            .unwrap(),
        if is_ascii { "ASCII" } else { "BINARY" }
    );
}

#[test]
fn encrypts_via_passphrase() {
    let mut context = MockProcessContext::new();
    context.properties.insert("Passphrase", "password");

    let input_ff = MockFlowFile::with_content("foo".as_bytes());
    let mut input_stream = input_ff.get_stream();
    let processor =
        EncryptContentPGP::schedule(&mut context, &MockLogger::new()).expect("should schedule");
    let transformed_ff = processor
        .transform(
            &mut context,
            &input_ff,
            &mut input_stream,
            &MockLogger::new(),
        )
        .expect("should transform");

    assert_content(&transformed_ff, false);
}

fn public_key_service() -> PGPPublicKeyService {
    let mut context = MockControllerServiceContext::new();
    context.properties.insert(
        "Keyring File".to_string(),
        test_utils::get_test_key_path("keyring.asc"),
    );
    let service = PGPPublicKeyService::enable(&context, &MockLogger::new()).expect("should enable");
    service
}

#[test]
fn encrypts_ascii_for_alice() {
    let mut context = MockProcessContext::new();
    context.properties.extend([
        ("Public Key Service", "my_controller_service"),
        ("Public Key Search", "Alice"),
        ("File Encoding", "ASCII"),
    ]);

    context.controller_services.insert(
        "my_controller_service".to_string(),
        Box::new(public_key_service()),
    );

    let input_ff = MockFlowFile::with_content("foo".as_bytes());
    let mut input_stream = input_ff.get_stream();
    let processor =
        EncryptContentPGP::schedule(&mut context, &MockLogger::new()).expect("should schedule");
    let transformed_ff = processor
        .transform(
            &mut context,
            &input_ff,
            &mut input_stream,
            &MockLogger::new(),
        )
        .expect("should transform");

    assert_content(&transformed_ff, true);
}

#[test]
fn encrypts_binary_for_bob() {
    let mut context = MockProcessContext::new();
    context.properties.extend([
        ("Public Key Service", "my_controller_service"),
        ("Public Key Search", "Bob"),
        ("File Encoding", "BINARY"),
    ]);

    context.controller_services.insert(
        "my_controller_service".to_string(),
        Box::new(public_key_service()),
    );

    let input_ff = MockFlowFile::with_content("foo".as_bytes());
    let mut input_stream = input_ff.get_stream();
    let processor =
        EncryptContentPGP::schedule(&mut context, &MockLogger::new()).expect("should schedule");
    let transformed_ff = processor
        .transform(
            &mut context,
            &input_ff,
            &mut input_stream,
            &MockLogger::new(),
        )
        .expect("should transform");

    assert_content(&transformed_ff, false);
}

#[test]
fn cannot_encrypt_for_carol() {
    let mut context = MockProcessContext::new();
    context.properties.extend([
        ("Public Key Service", "my_controller_service"),
        ("Public Key Search", "Carol"),
    ]);

    context.controller_services.insert(
        "my_controller_service".to_string(),
        Box::new(public_key_service()),
    );

    let input_ff = MockFlowFile::with_content("foo".as_bytes());
    let mut input_stream = input_ff.get_stream();
    let processor =
        EncryptContentPGP::schedule(&mut context, &MockLogger::new()).expect("should schedule");
    let transformed_ff = processor
        .transform(
            &mut context,
            &input_ff,
            &mut input_stream,
            &MockLogger::new(),
        )
        .expect("should transform");

    assert_eq!(transformed_ff.target_relationship(), &FAILURE);
    assert!(transformed_ff.new_content().is_none());
    assert!(transformed_ff.attributes_to_add().is_empty());
}
