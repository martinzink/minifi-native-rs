@SUPPORTS_WINDOWS
Feature: Testing custom and default metrics

  Scenario: GetFileRs's custom metrics
    Given a GetFileRs processor with the "Input Directory" property set to "/tmp/input"
    And GetFileRs's success relationship is auto-terminated
    And a directory at "/tmp/input" has a file ("input.txt") with the content "the brown fox jumps over the lazy dog"
    And MiNiFi logs processor metrics

    When the MiNiFi instance starts up

    Then the Minifi logs contain the following message: ""InputBytes": "37"" in less than 10 seconds
    And the Minifi logs contain the following message: ""AcceptedFiles": "1"" in less than 10 seconds
    And the Minifi logs contain the following message: ""TransferredToSuccess": "1"" in less than 10 seconds
    And the Minifi logs contain the following message: ""OnTriggerInvocations": "1"" in less than 10 seconds
    And the Minifi logs contain the following message: ""BytesRead": "0"" in less than 10 seconds
    And the Minifi logs contain the following message: ""BytesWritten": "37"" in less than 10 seconds
    And the Minifi logs do not contain errors
    And the Minifi logs do not contain warnings
