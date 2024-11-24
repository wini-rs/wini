#!/usr/bin/env bash

# Build the template

tmp_dir="$(mktemp -d)"
cd "$tmp_dir"
git clone https://codeberg.org/wini/wini-template
cd -
cd "$tmp_dir/wini-template"
yes | rm -rf *
cd -
cp -r ./wini/* "$tmp_dir/wini-template"
cp -r ./wini/.gitignore "$tmp_dir/wini-template"
cd -
gitui
