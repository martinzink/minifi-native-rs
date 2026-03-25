# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

Feature: Sending data from MiNiFi-C++ to an AWS server
  In order to transfer data to interact with AWS servers
  As a user of MiNiFi
  I need to have PutS3ObjectRs and DeleteS3Object processors

  Scenario: A MiNiFi instance transfers encoded data to s3
    Given a GetFile processor with the "Input Directory" property set to "/tmp/input"
    And a directory at "/tmp/input" has a file with the content "LH_O#L|FD<FASD{FO#@$#$%^ \"#\"$L%:\"@#$L\":test_data#$#%#$%?{\"F{"
    And a PutS3ObjectRs processor set up to communicate with an s3 server
    And a PutFile processor with the "Directory" property set to "/tmp/output"
    And a LogAttribute processor with the name "log_before"
    And a LogAttribute processor with the name "log_after"
    And the "Log Payload" property of the log_before processor is set to "true"
    And the "Log Payload" property of the log_after processor is set to "true"
    And the "Log Level" property of the log_before processor is set to "Info"
    And the "Log Level" property of the log_after processor is set to "Info"

    And the "success" relationship of the GetFile processor is connected to the log_before
    And the "success" relationship of the log_before processor is connected to the PutS3ObjectRs
    And the "success" relationship of the PutS3ObjectRs processor is connected to the log_after
    And the "success" relationship of the log_after processor is connected to the PutFile
    And the "failure" relationship of the PutS3ObjectRs processor is connected to the PutS3ObjectRs
    And PutFile's success relationship is auto-terminated

    And the s3 server starts up

    When the MiNiFi instance starts up

    Then a single file with the content "LH_O#L|FD<FASD{FO#@$#$%^ \"#\"$L%:\"@#$L\":test_data#$#%#$%?{\"F{" is placed in the "/tmp/output" directory in less than 20 seconds
    And the object on the s3 server is "LH_O#L|FD<FASD{FO#@$#$%^ \"#\"$L%:\"@#$L\":test_data#$#%#$%?{\"F{"
    And the object content type on the s3 server is "application/octet-stream" and the object metadata matches use metadata
