use super::PrivateKeyService;
use crate::test_utils::get_test_key_path;
use minifi_native::MinifiError::ControllerServiceError;
use minifi_native::{ControllerService, MockControllerServiceContext, MockLogger};

#[test]
fn default_fails() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let context = MockControllerServiceContext::new();

    assert_eq!(
        controller_service.enable(&context),
        Err(ControllerServiceError("Could not load any valid keys"))
    );
}

#[test]
fn corrupted_binary_keyring_file() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context
        .properties
        .insert("Key File".to_string(), get_test_key_path("garbage.gpg"));

    assert_eq!(
        controller_service.enable(&context),
        Err(ControllerServiceError("Could not load any valid keys"))
    );
}

#[test]
fn armored_public_key_file() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context.properties.insert(
        "Key File".to_string(),
        get_test_key_path("private_mistake.asc"),
    );

    assert_eq!(
        controller_service.enable(&context),
        Err(ControllerServiceError("Could not load any valid keys"))
    );
}

#[test]
fn corrupted_armored_key_file() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context.properties.insert(
        "Key File".to_string(),
        get_test_key_path("truncated_private.asc"),
    );

    assert_eq!(
        controller_service.enable(&context),
        Err(ControllerServiceError("Could not load any valid keys"))
    );
}

#[test]
fn non_existent_keyfile() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context.properties.insert(
        "Key File".to_string(),
        get_test_key_path("non_existent.asc"),
    );

    assert_eq!(
        controller_service.enable(&context),
        Err(ControllerServiceError("Could not load any valid keys"))
    );
}

#[test]
fn single_armored_key_file() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context
        .properties
        .insert("Key File".to_string(), get_test_key_path("alice_private.asc"));

    assert_eq!(controller_service.enable(&context), Ok(()));
    assert!(controller_service.get_secret_key("Alice").is_some());
    assert!(controller_service.get_secret_key("alice@example.com").is_some());

    assert!(controller_service.get_secret_key("Bob").is_none());
    assert!(controller_service.get_secret_key("Carol").is_none());
}

#[test]
fn single_binary_key_file() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context
        .properties
        .insert("Key File".to_string(), get_test_key_path("alice_private.gpg"));

    assert_eq!(controller_service.enable(&context), Ok(()));
    assert!(controller_service.get_secret_key("A").is_some());
    assert!(controller_service.get_secret_key("Alice").is_some());
    assert!(
        controller_service
            .get_secret_key("Alice <alice@example.com>")
            .is_some()
    );

    assert!(controller_service.get_secret_key("<Alice>").is_none());

    assert!(controller_service.get_secret_key("Bob").is_none());
    assert!(controller_service.get_secret_key("Carol").is_none());
}

#[test]
fn armored_keyring_key_file() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context
        .properties
        .insert("Key File".to_string(), get_test_key_path("secret_keyring.asc"));

    assert_eq!(controller_service.enable(&context), Ok(()));
    assert!(controller_service.get_secret_key("Alice").is_some());
    assert!(controller_service.get_secret_key("Bob").is_some());
    assert!(controller_service.get_secret_key("bob@home.io").is_some());
    assert!(controller_service.get_secret_key("bob@work.com").is_some());
    assert!(controller_service.get_secret_key("Carol").is_none());
}

#[test]
fn binary_keyring_key_file() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();
    context
        .properties
        .insert("Key File".to_string(), get_test_key_path("secret_keyring.gpg"));

    assert_eq!(controller_service.enable(&context), Ok(()));
    assert!(controller_service.get_secret_key("Alice").is_some());
    assert!(controller_service.get_secret_key("Bob").is_some());
    assert!(controller_service.get_secret_key("bob@home.io").is_some());
    assert!(controller_service.get_secret_key("bob@work.com").is_some());
    assert!(controller_service.get_secret_key("Carol").is_none());
}

#[test]
fn armored_keyring() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();

    let file_content =
        std::fs::read_to_string(get_test_key_path("secret_keyring.asc")).expect("required for test");

    context
        .properties
        .insert("Key".to_string(), file_content);

    assert_eq!(controller_service.enable(&context), Ok(()));
    assert!(controller_service.get_secret_key("Alice").is_some());
    assert!(controller_service.get_secret_key("Bob").is_some());
    assert!(controller_service.get_secret_key("bob@home.io").is_some());
    assert!(controller_service.get_secret_key("bob@work.com").is_some());
    assert!(controller_service.get_secret_key("Carol").is_none());
}

#[test]
fn armored_single_key() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();

    let file_content =
        std::fs::read_to_string(get_test_key_path("alice_private.asc")).expect("required for test");

    context
        .properties
        .insert("Key".to_string(), file_content);

    assert_eq!(controller_service.enable(&context), Ok(()));
    assert!(controller_service.get_secret_key("Alice").is_some());
    assert!(controller_service.get_secret_key("Bob").is_none());
    assert!(controller_service.get_secret_key("Carol").is_none());
}

#[test]
fn corrupted_armored_key() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();

    let file_content =
        std::fs::read_to_string(get_test_key_path("truncated_private.asc")).expect("required for test");

    context
        .properties
        .insert("Key".to_string(), file_content);

    assert_eq!(
        controller_service.enable(&context),
        Err(ControllerServiceError("Could not load any valid keys"))
    );
}

#[test]
fn public_ascii_key() {
    let mut controller_service = PrivateKeyService::new(MockLogger::new());
    let mut context = MockControllerServiceContext::new();

    let file_content = std::fs::read_to_string(get_test_key_path("alice.asc"))
        .expect("required for test");

    context
        .properties
        .insert("Key".to_string(), file_content);

    assert_eq!(
        controller_service.enable(&context),
        Err(ControllerServiceError("Could not load any valid keys"))
    );
}
