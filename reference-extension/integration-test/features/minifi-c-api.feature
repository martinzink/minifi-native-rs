Feature: Test Minifi Native C Api capabilities

  Background: The reference library is successfully built on linux

  Scenario: The rust library is loaded into minifi
    Given the built rust extension library is inside minifi's extension folder
    And log property "logger.org::apache::nifi::minifi::core::extension::ExtensionManager" is set to "TRACE,stderr"

    When the MiNiFi instance starts up

    Then the Minifi logs contain the following message: "Successfully initialized extension 'minifi-rust'" in less than 20 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings

  Scenario: Simple GenerateFlowFileRs -> PutFileRs
    Given the built rust extension library is inside minifi's extension folder
    And a GenerateFlowFileRs processor with the "Custom Text" property set to "Ferris the crab"
    And the "Data Format" property of the GenerateFlowFileRs processor is set to "Text"
    And the "Unique FlowFiles" property of the GenerateFlowFileRs processor is set to "false"
    And a PutFileRs processor with the "Directory" property set to "/tmp/output"
    And the "success" relationship of the GenerateFlowFileRs processor is connected to the PutFileRs
    And PutFileRs's success relationship is auto-terminated

    When the MiNiFi instance starts up

    Then at least one file with the content "Ferris the crab" is placed in the "/tmp/output" directory in less than 10 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings

  Scenario: Simple GetFileRs -> PutFileRs
    Given the built rust extension library is inside minifi's extension folder
    And a GetFileRs processor with the "Input Directory" property set to "/tmp/input"
    And a PutFileRs processor with the "Directory" property set to "/tmp/output"
    And the "success" relationship of the GetFileRs processor is connected to the PutFileRs
    And PutFileRs's success relationship is auto-terminated
    And PutFileRs's failure relationship is auto-terminated
    And a directory at "/tmp/input" has a file ("test_file.log") with the content "test content"
    And log property "logger.rs::PutFileRs" is set to "TRACE,stderr"
    And log property "logger.rs::GetFileRs" is set to "TRACE,stderr"

    When the MiNiFi instance starts up

    Then there is a file with "test content" content at /tmp/output/test_file.log in less than 10 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings

  Scenario Outline: The LogAttributeRs can read and log FlowFile content
    Given the built rust extension library is inside minifi's extension folder
    And a GenerateFlowFileRs processor with the "Custom Text" property set to "<custom_text>"
    And the "Data Format" property of the GenerateFlowFileRs processor is set to "Text"
    And the "Unique FlowFiles" property of the GenerateFlowFileRs processor is set to "false"
    And a LogAttributeRs processor with the "Log Level" property set to "<log_level>"
    And the "Log Payload" property of the LogAttributeRs processor is set to "true"
    And the "success" relationship of the GenerateFlowFileRs processor is connected to the LogAttributeRs
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
    And a GenerateFlowFileRs processor with the "Custom Text" property set to "${invalid_attribute}"
    And the "Data Format" property of the GenerateFlowFileRs processor is set to "Text"
    And the "Unique FlowFiles" property of the GenerateFlowFileRs processor is set to "false"
    And a LogAttributeRs processor with the "Log Level" property set to "Critical"
    And the "success" relationship of the GenerateFlowFileRs processor is connected to the LogAttributeRs
    And LogAttributeRs's success relationship is auto-terminated

    When the MiNiFi instance starts up

    Then Waits for 3 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings
