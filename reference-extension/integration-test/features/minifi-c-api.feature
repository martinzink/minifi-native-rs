Feature: Test Minifi Native C Api capabilities

  Background: The reference library is successfully built on linux

  Scenario: The rust library is loaded into minifi
    Given the built rust extension library is inside minifi's extension folder
    And log property "logger.org::apache::nifi::minifi::core::extension::ExtensionManager" is set to "TRACE,stderr"

    When the MiNiFi instance starts up

    Then the Minifi logs contain the following message: "Successfully initialized extension 'minifi-rust'" in less than 200 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings

  Scenario: The SimpleSourceProcessor writes property value into FlowFiles
    Given the built rust extension library is inside minifi's extension folder
    And a SimpleSourceProcessor processor with the "Content" property set to "Ferris the crab"
    And a PutFile processor with the "Directory" property set to "/tmp/output"
    And the "success" relationship of the SimpleSourceProcessor processor is connected to the PutFile
    And PutFile's success relationship is auto-terminated

    When the MiNiFi instance starts up

    Then at least one file with the content "Ferris the crab" is placed in the "/tmp/output" directory in less than 10 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings

  Scenario Outline: The LogAttributeRs can read and log FlowFile content
    Given the built rust extension library is inside minifi's extension folder
    And a GenerateFlowFile processor with the "Custom Text" property set to "<custom_text>"
    And the "Data Format" property of the GenerateFlowFile processor is set to "Text"
    And the "Unique FlowFiles" property of the GenerateFlowFile processor is set to "false"
    And a LogAttributeRs processor with the "Log Level" property set to "<log_level>"
    And the "Log Payload" property of the LogAttributeRs processor is set to "true"
    And the "success" relationship of the GenerateFlowFile processor is connected to the LogAttributeRs
    And LogAttributeRs's success relationship is auto-terminated

    When the MiNiFi instance starts up

    Then the Minifi logs contain the following message: "<expected_log_1>" in less than 20 seconds
    And the Minifi logs contain the following message: "<expected_log_2>" in less than 1 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings
    Examples:
      | custom_text   | log_level | expected_log_1                   | expected_log_2 |
      | Keith the rat | Critical  | [critical] Logging for flow file | Keith the rat  |
      | Keith the rat | Info      | [info] Logging for flow file     | Keith the rat  |

  Scenario: The Api handles empty flow-files
    Given the built rust extension library is inside minifi's extension folder
    And a SimpleSourceProcessor processor with the "Content" property set to "${invalid_attribute}"
    And a LogAttributeRs processor with the "Log Level" property set to "Critical"
    And the "success" relationship of the SimpleSourceProcessor processor is connected to the LogAttributeRs
    And LogAttributeRs's success relationship is auto-terminated

    When the MiNiFi instance starts up

    Then Waits for 5 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings
