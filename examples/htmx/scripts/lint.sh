#!/usr/bin/env bash

# This script is in charge of linting files

source ./scripts/log.sh

set -euo pipefail


info "Compiling scss..."
./scripts/scss.sh &> /dev/null
info "=== Biome linter ==="
bunx biome lint
info "=== Rust clippy ==="
cargo clippy
