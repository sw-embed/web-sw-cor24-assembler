#!/bin/bash
# Production build: compile WASM and output to pages/
# Usage: ./build.sh [--clean]

set -e

if [ "$1" = "--clean" ]; then
    echo "Cleaning build artifacts..."
    cargo clean
fi

echo "Building release WASM..."
trunk build --release

echo "Build complete. Production assets in pages/"
