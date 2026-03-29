#!/usr/bin/env bash
set -euo pipefail

# Build the web app for GitHub Pages deployment
# Output goes to pages/ directory

trunk build --release
