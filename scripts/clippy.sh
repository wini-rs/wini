#!/usr/bin/env bash

set -euo pipefail

for dir in wini wini-cli wini-maud wini-maud-macros; do
    echo "=== [$dir] ==="
    cd "$dir"
    cargo clippy
    cd -
done
