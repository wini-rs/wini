#!/usr/bin/env bash

rm -rf scripts/*.nu

new_justfile=$(head -n -3 justfile)
echo "$new_justfile" > justfile

rm scripts/on-install.sh
