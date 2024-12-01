#!/usr/bin/env bash

# This script is in charge of updating the template


source ./scripts/log.sh

set -euo pipefail

on_interrupt() {
    git remote remove wini-template
}

trap on_interrupt SIGINT


if ! git status | rg 'nothing to commit, working tree clean'; then
    error "Commit your current changes before updating wini"
    exit 1
fi

origin="$(yq -p toml ".origin" < ./wini.toml)"

remote_url="$(yq ".remote_url" <<< "$origin")"

if [ "$remote_url" = 'NONE' ]; then
    error "No remote repository. The project was created using a local repository."
    exit 1
fi


git remote add wini-template "$remote_url"
git fetch wini-template
git cherry-pick "$(yq ".last_commit_hash" <<< "$origin")"..wini-template/"$(yq ".branch" <<< "$origin")"
git remote remove wini-template
