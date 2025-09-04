@CORE
Feature: Test Minifi Native C Api capabilities

  Scenario: The rust library is loaded into minifi
    Given the built rust extension library is inside minifi's extension folder
    And log property "logger.org::apache::nifi::minifi::core::extension::ExtensionManager" is set to "TRACE,stderr"

    When the MiNiFi instance starts up

    Then the Minifi logs contain the following message: "libminifi-rust.so as CApi extension" in less than 10 seconds
