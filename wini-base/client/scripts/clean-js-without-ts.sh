#!/usr/bin/env bash

# Clean javascript files that don't have an associated typescript file in `./src`

set -euo pipefail


js_files="$(fd -e js . ./src -HI)"

for js_file in $js_files; do
    if [ ! -f "${js_file/%.js/.ts}" ]; then
        rm "$js_file"
        echo "Removed $js_file"
    fi
done
