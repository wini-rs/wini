#!/usr/bin/env bash

rm -rf scripts/*.nu

new_justfile=$(head -n -3 justfile)
echo "$new_justfile" > justfile

# IFFEAT ssr
rm src/shared/wini/ssg.rs
# ENDIF
rm scripts/on-install.sh
