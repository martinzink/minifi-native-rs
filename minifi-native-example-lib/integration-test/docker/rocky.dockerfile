# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements. See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership. The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License. You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied. See the License for the
# specific language governing permissions and limitations
# under the License.
#
ARG BASE_IMAGE="rockylinux:8"

FROM ${BASE_IMAGE} AS build
LABEL maintainer="Martin Zink <martinzink@apache.org>"

RUN dnf install -y clang
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV BUILD_DIR /opt/minifi-native-rs
COPY .. ${BUILD_DIR}
RUN cd ${BUILD_DIR}/minifi-native-example-lib && /root/.cargo/bin/cargo build --release
