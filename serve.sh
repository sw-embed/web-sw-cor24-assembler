#!/bin/bash
# Start trunk development server on port 7401
# Navigate to http://localhost:7401/cor24-rs/ to view the app
# Usage: ./serve.sh [--clean]

set -e

if [ "$1" = "--clean" ]; then
    echo "Cleaning build artifacts..."
    cargo clean
    shift
fi

trunk serve --release --port 7401 "$@"
