#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/../../.."
pwd
docker buildx build -f reference-extension/integration-test/docker/rocky.dockerfile . -t reference-extension-build:latest

docker create --name temp-container reference-extension-build:latest

docker cp temp-container:/opt/minifi-native-rs/reference-extension/target/release/libreference_extension.so ./reference-extension/integration-test/features/linux_so/

docker rm temp-container