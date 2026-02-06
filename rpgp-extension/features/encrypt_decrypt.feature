Feature: Test PGP extension's encryption and decryption capabilities

  Background: The pgp library is successfully built on linux

  Scenario: The pgp library is loaded into minifi
    Given the built rust extension library is inside minifi's extension folder
    And log property "logger.org::apache::nifi::minifi::core::extension::ExtensionManager" is set to "TRACE,stderr"
    And log property "logger.org::apache::nifi::minifi::core::ClassLoader" is set to "TRACE,stderr"

    When the MiNiFi instance starts up

    Then the Minifi logs contain the following message: "libminifi-pgp.so as CApi extension" in less than 10 seconds
    And the Minifi logs contain the following message: "Registering class 'EncryptContentPGP' at '/rpgp-extension'" in less than 1 seconds
    And the Minifi logs contain the following message: "Registering class 'DecryptContentPGP' at '/rpgp-extension'" in less than 1 seconds
    And the Minifi logs contain the following message: "Registering class 'PgpPublicKeyService' at '/rpgp-extension'" in less than 1 seconds
    And the Minifi logs contain the following message: "Registering class 'PgpPrivateKeyService' at '/rpgp-extension'" in less than 1 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings

  Scenario: GetFile -> Encrypt -> Decrypt -> PutFile
    Given the built rust extension library is inside minifi's extension folder

    And a GetFile processor with the "Input Directory" property set to "/tmp/input"
    And an EncryptContentPGP processor with a PgpPublicKeyService is set up
    And a DecryptContentPGP processor with a PgpPrivateKeyService is set up
    And a PutFile processor with the "Directory" property set to "/tmp/output"

    And the "success" relationship of the GetFile processor is connected to the EncryptContentPGP
    And the "success" relationship of the EncryptContentPGP processor is connected to the DecryptContentPGP
    And the "success" relationship of the DecryptContentPGP processor is connected to the PutFile

    And PutFile's success relationship is auto-terminated
    And PutFile's failure relationship is auto-terminated

    And a directory at "/tmp/input" has a file ("test_file.log") with the content "test content"

    When the MiNiFi instance starts up

    Then at least one file with the content "test content" is placed in the "/tmp/output" directory in less than 10 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings
