#!/usr/bin/env bash
set -e

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "$PROJECT_ROOT"

DOCKERFILE="minifi_rs_behave/rocky.dockerfile"
TARGET_DIR="target/release"

SOURCE_PATHS="src Cargo.toml Cargo.lock $DOCKERFILE"

mkdir -p "$TARGET_DIR"

docker buildx build \
  -f "$DOCKERFILE" \
  --target bin-export \
  --output type=local,dest="$TARGET_DIR" \
  .

echo "Build complete. Artifacts updated in $TARGET_DIR"

