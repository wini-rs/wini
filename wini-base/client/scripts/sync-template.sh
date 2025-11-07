#!/usr/bin/env bash

# This script is in charge of updating the template


source ./scripts/log.sh

set -euo pipefail

on_interrupt() {
    git remote remove wini-template
}

trap on_interrupt SIGINT TERM EXIT


if ! git status | rg 'nothing to commit, working tree clean'; then
    error "Commit your current changes before updating wini"
    exit 1
fi

origin="$(yq -p toml ".origin" < ./wini.toml)"

last_commit_hash="$(yq ".last_commit_hash" <<< "$origin")"
remote_url="$(yq ".remote_url" <<< "$origin")"
branch="$(yq ".branch" <<< "$origin")"

if [ "$remote_url" = 'NONE' ]; then
    error "No remote repository. The project was created using a local repository."
    exit 1
fi


git remote add wini-template "$remote_url"
git fetch wini-template

# Check for coherent last_commit_hash
if ! git merge-base --is-ancestor "$last_commit_hash" "wini-template/$branch"; then
    error 'Invalid `last_commit_hash`: doesn'"'t exists in remote wini-template/$branch"
    git remote remove wini-template
    exit 1
fi

if ! git cherry-pick "$last_commit_hash"..wini-template/"$branch"; then
    git remote remove wini-template || true
    error 'Cherry-pick failed; please resolve conflicts and after that, do `wini sync-commit-hash`'
    exit 1
fi

git remote remove wini-template || true

just sync-commit-hash
