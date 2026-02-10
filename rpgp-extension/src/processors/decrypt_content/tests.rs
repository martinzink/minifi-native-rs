use crate::controller_services::private_key_service;
use crate::controller_services::private_key_service::PrivateKeyService;
use crate::processors::decrypt_content::DecryptContent;
use crate::test_utils;
use crate::test_utils::get_test_message;
use minifi_native::{
    ControllerService, FlowFileTransform, MockControllerServiceContext, MockFlowFile, MockLogger,
    MockProcessContext, Schedule,
};

#[test]
fn fails_to_schedule_by_default() {
    let decrypt_content = DecryptContent::schedule(&MockProcessContext::new(), &MockLogger::new());
    assert!(decrypt_content.is_err());
}

#[test]
fn schedules_with_password() {
    let mut context = MockProcessContext::new();
    context.properties.insert(
        super::properties::SYMMETRIC_PASSWORD.name.to_string(),
        "my_secret_password".to_string(),
    );
    let decrypt_content = DecryptContent::schedule(&context, &MockLogger::new());
    assert!(decrypt_content.is_ok());
}

#[test]
fn schedules_with_controller() {
    let mut context = MockProcessContext::new();
    context.properties.insert(
        super::properties::PRIVATE_KEY_SERVICE.name.to_string(),
        "my_private_key_service".to_string(),
    );
    let decrypt_content = DecryptContent::schedule(&context, &MockLogger::new());
    assert!(decrypt_content.is_ok());
}

#[derive(Copy, Clone)]
struct PrivateKeyData {
    key_filename: &'static str,
    passphrase: Option<&'static str>,
}

impl PrivateKeyData {
    fn into_controller(self) -> PrivateKeyService {
        let mut controller_service = PrivateKeyService::new(MockLogger::new());
        let mut context = MockControllerServiceContext::new();
        use private_key_service::properties::{KEY_FILE, KEY_PASSPHRASE};
        context.properties.insert(
            KEY_FILE.name,
            test_utils::get_test_key_path(self.key_filename),
        );

        if let Some(passphrase) = self.passphrase {
            context.properties.insert(KEY_PASSPHRASE.name, passphrase);
        }

        assert_eq!(controller_service.enable(&context), Ok(()));
        controller_service
    }
}

fn test_decryption(
    message_file_name: &str,
    private_key_data: Option<PrivateKeyData>,
    symmetric_password: Option<&'static str>,
    expected_result: Result<&[u8], ()>,
) {
    let mut processor_context = MockProcessContext::new();
    if let Some(private_key) = private_key_data {
        processor_context.controller_services.insert(
            "my_private_key_service".to_string(),
            Box::new(private_key.into_controller()),
        );
        processor_context.properties.insert(
            super::properties::PRIVATE_KEY_SERVICE.name.to_string(),
            "my_private_key_service".to_string(),
        );
    }
    if let Some(symmetric_password) = symmetric_password {
        processor_context.properties.insert(
            super::properties::SYMMETRIC_PASSWORD.name.to_string(),
            symmetric_password.to_string(),
        );
    }

    let decrypt_content = DecryptContent::schedule(&processor_context, &MockLogger::new())
        .expect("Should schedule without any properties");
    let res = decrypt_content
        .transform(
            &mut processor_context,
            MockFlowFile::new(),
            |_ff| Some(get_test_message(message_file_name)),
            &MockLogger::new(),
        )
        .expect("Should be able to transform");

    match expected_result {
        Ok(result_bytes) => {
            assert_eq!(res.target_relationship(), &super::relationships::SUCCESS);
            assert_eq!(res.new_content().unwrap(), result_bytes);
        }
        Err(_) => {
            assert_eq!(res.target_relationship(), &super::relationships::FAILURE);
            assert!(res.new_content().is_none());
        }
    }
}

#[test]
fn decrypts_with_password() {
    test_decryption(
        "password_encrypted_foo.gpg",
        None,
        Some("my_secret_password"),
        Ok("foo\n".as_bytes()),
    );
    test_decryption(
        "password_encrypted_foo.asc",
        None,
        Some("my_secret_password"),
        Ok("foo\n".as_bytes()),
    );
    test_decryption(
        "foo_for_alice.gpg",
        None,
        Some("my_secret_password"),
        Err(()),
    );
    test_decryption(
        "foo_for_alice.asc",
        None,
        Some("my_secret_password"),
        Err(()),
    );
}

#[test]
fn decrypts_for_alice() {
    let alice_private_key_data = PrivateKeyData {
        key_filename: "alice_private.asc",
        passphrase: Some("whiterabbit"),
    };

    test_decryption(
        "foo_for_alice.asc",
        Some(alice_private_key_data),
        None,
        Ok("foo\n".as_bytes()),
    );

    test_decryption(
        "foo_for_alice.gpg",
        Some(alice_private_key_data),
        None,
        Ok("foo\n".as_bytes()),
    );

    test_decryption(
        "password_encrypted_foo.gpg",
        Some(alice_private_key_data),
        None,
        Err(()),
    );

    test_decryption(
        "password_encrypted_foo.asc",
        Some(alice_private_key_data),
        None,
        Err(()),
    );
}
