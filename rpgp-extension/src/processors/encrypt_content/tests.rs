use super::*;
use crate::test_utils;
use minifi_native::{
    ControllerService, MockControllerServiceContext, MockFlowFile, MockLogger, MockProcessContext,
    TransformedFlowFile,
};

fn encrypt_with_processor(
    mut context: MockProcessContext,
) -> TransformedFlowFile<'static, MockFlowFile> {
    let processor = EncryptContent::schedule(&context, &MockLogger::new())
        .expect("Should schedule without any properties"); // TODO(mzink) maybe it shouldnt?
    let res = processor
        .transform(
            &mut context,
            MockFlowFile::new(),
            |_ff| return Some("foo".as_bytes().to_vec()),
            &MockLogger::new(),
        )
        .expect("Should be able to transform");
    res
}

#[test]
fn schedules_but_fails_to_encrypt_with_defaults() {
    let transformed_ff = encrypt_with_processor(MockProcessContext::new());
    assert_eq!(transformed_ff.target_relationship(), &FAILURE);
}

#[test]
fn encrypts_via_passphrase() {
    let mut context = MockProcessContext::new();
    context.properties.insert("Passphrase", "password");

    let transformed_ff = encrypt_with_processor(context);

    assert_eq!(transformed_ff.target_relationship(), &SUCCESS);
    assert!(transformed_ff.new_content().is_some());
    assert_eq!(
        transformed_ff
            .attributes_to_add()
            .get("pgp.file.encoding")
            .unwrap(),
        "BINARY"
    );
}

fn public_key_service() -> PublicKeyService {
    let mut controller_service = PublicKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context.properties.insert(
        "Keyring File".to_string(),
        test_utils::get_test_key_path("keyring.asc"),
    );
    assert_eq!(controller_service.enable(&context), Ok(()));
    controller_service
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

    let transformed_ff = encrypt_with_processor(context);
    assert_eq!(transformed_ff.target_relationship(), &SUCCESS);
    assert!(transformed_ff.new_content().is_some());
    assert!(transformed_ff.new_content().unwrap().is_ascii());
    assert_eq!(
        transformed_ff
            .attributes_to_add()
            .get("pgp.file.encoding")
            .unwrap(),
        "ASCII"
    );
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

    let transformed_ff = encrypt_with_processor(context);
    assert_eq!(transformed_ff.target_relationship(), &SUCCESS);
    assert!(transformed_ff.new_content().is_some());
    assert!(!transformed_ff.new_content().unwrap().is_ascii());
    assert_eq!(
        transformed_ff
            .attributes_to_add()
            .get("pgp.file.encoding")
            .unwrap(),
        "BINARY"
    );
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

    let transformed_ff = encrypt_with_processor(context);
    assert_eq!(transformed_ff.target_relationship(), &FAILURE);
    assert!(transformed_ff.new_content().is_none());
    assert!(transformed_ff.attributes_to_add().is_empty());
}
