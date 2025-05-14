#!/bin/bash
set -e

echo "Starting build of all projects..."

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"
DEMO_DIR="$ROOT_DIR/demo"

echo "Building composition..."
cd "$ROOT_DIR/composition"
cargo build --release

echo "Copying composition binaries..."
cp target/release/host "$DEMO_DIR/host_comp"
cp target/release/verifier "$DEMO_DIR/verifier_comp"

echo "Building imgTransformations..."
cd "$ROOT_DIR/imgTransformations"
cargo build --release

echo "Copying imgTransformations binaries..."
cp target/release/host "$DEMO_DIR/host_img"
cp target/release/verifier "$DEMO_DIR/verifier_img"

echo "All builds complete. Binaries are in demo/"