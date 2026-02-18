use super::*;
use crate::test_utils;
use minifi_native::{
    Content, EnableControllerService, MockControllerServiceContext, MockFlowFile, MockLogger,
    MockProcessContext, TransformedFlowFile,
};

fn encrypt_with_processor(
    context: &'_ mut MockProcessContext,
) -> TransformedFlowFile<'_, MockFlowFile> {
    let processor =
        EncryptContentPGP::schedule(context, &MockLogger::new()).expect("should schedule");
    let res = processor
        .transform(
            context,
            MockFlowFile::new(),
            |_ff| return Some("foo".as_bytes().to_vec()),
            &MockLogger::new(),
        )
        .expect("Should be able to transform");
    res
}

#[test]
fn cannot_schedule_without_password_or_public_key() {
    assert!(EncryptContentPGP::schedule(&MockProcessContext::new(), &MockLogger::new()).is_err());
}

fn assert_content(transform_result: &TransformedFlowFile<MockFlowFile>, is_ascii: bool) {
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

    let transformed_ff = encrypt_with_processor(&mut context);

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

    let transformed_ff = encrypt_with_processor(&mut context);
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

    let transformed_ff = encrypt_with_processor(&mut context);
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

    let transformed_ff = encrypt_with_processor(&mut context);
    assert_eq!(transformed_ff.target_relationship(), &FAILURE);
    assert!(transformed_ff.new_content().is_none());
    assert!(transformed_ff.attributes_to_add().is_empty());
}
