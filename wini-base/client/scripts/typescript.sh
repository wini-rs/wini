#!/usr/bin/env bash

# This script is in charge of compiling typescript files to javascript.

set -euo pipefail


mapfile -t ts_files < <(fd -e ts . ./src)

for file in "${ts_files[@]}"; do
    bun build --no-bundle "$file" -e '*' --minify-syntax --minify-whitespace > "${file/%.ts/.js}"
    sed -i 's/import\s*\([A-Za-z0-9_\*]*\)\?\s*,\?\s*\({[^}]*}\)\?\s*\(from\)\?\s*\(['"'"'"][^'"'"'"]*['"'"'"]\);\?//g' "${file/%.ts/.js}"
done

bun build --no-bundle "./public/helpers.js" -e '*' --minify-syntax --minify-whitespace > "./public/helpers.min.js"
