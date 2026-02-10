#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."
pwd

mkdir ./docker_builder/target

docker buildx build -f docker_builder/rocky.dockerfile . -t minifi-rust-extension-build:latest

docker create --name temp-container minifi-rust-extension-build:latest

docker cp temp-container:/opt/minifi-native-rs/target/release/librust_reference_extension.so ./docker_builder/target
docker cp temp-container:/opt/minifi-native-rs/target/release/librpgp_extension.so ./docker_builder/target

docker rm temp-container