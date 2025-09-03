@CORE
Feature: Test Minifi Native C Api capabilities

  Scenario: The rust library is loaded into minifi
    Given the built rust extension library is inside minifi's extension folder

    When the MiNiFi instance starts up

    Then the Minifi logs contain the following message: "Using plaintext FlowFileRepository" in less than 5 seconds
