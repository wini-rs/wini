#!/usr/bin/env bash

# This script is in charge of compiling scss files to javascript.

set -euo pipefail


mapfile -t scss_files < <(fd -e scss . src)

for file in "${scss_files[@]}"; do
    sass "$file:$(echo "$file" | sed 's/\.scss$/.css/g')" --no-source-map --style compressed
done
