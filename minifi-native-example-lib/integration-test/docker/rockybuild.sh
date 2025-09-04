#!/usr/bin/env bash
cd "$(dirname "$0")/../../.."
pwd
#docker buildx build -f minifi-native-example-lib/integration-test/docker/rocky.dockerfile . -t minifi-native-rs-example-lib-build:latest

docker create --name temp-container minifi-native-rs-example-lib-build:latest

docker cp temp-container:/opt/minifi-native-rs/minifi-native-example-lib/target/release/libminifi_native_example_lib.so ./minifi-native-example-lib/integration-test/features/linux_so

docker rm temp-container