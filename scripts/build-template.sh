#!/usr/bin/env bash

# Build the template

set -euox pipefail

tmp_dir="$(mktemp -d)"
cd "$tmp_dir"
git clone https://codeberg.org/wini/wini-template
cd -
cd "$tmp_dir/wini-template"
yes | rm -rf * || true
cd -
cp -r ./wini/* "$tmp_dir/wini-template"
cp -r ./wini/.gitignore "$tmp_dir/wini-template"
cp -r ./wini/.env "$tmp_dir/wini-template"
cd -
git add -f .env
gitui
git push origin main
git remote add github git@github.com:wini-rs/wini-template.git
git push github main
