#!/usr/bin/env bash
set -e

# 1. Establish the Project Root (Absolute Path)
# This ensures that no matter where you call the script from, it stays consistent.
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "$PROJECT_ROOT"

# Configuration (Paths relative to PROJECT_ROOT)
DOCKERFILE="behave_tester/rocky.dockerfile"
TARGET_DIR="target/release"

# 2. Define exactly what files trigger a rebuild
# We use relative paths from the PROJECT_ROOT
SOURCE_PATHS="src Cargo.toml Cargo.lock $DOCKERFILE"

mkdir -p "$TARGET_DIR"

# 5. Execute Build with direct export
# The '.' at the end tells docker the context is the PROJECT_ROOT
docker buildx build \
  -f "$DOCKERFILE" \
  --target bin-export \
  --output type=local,dest="$TARGET_DIR" \
  .

echo "Build complete. Artifacts updated in $TARGET_DIR"

