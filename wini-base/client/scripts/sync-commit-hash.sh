#!/usr/bin/env bash

# This script is in charge of syncing the `last_commit_hash` of `wini.toml`

source ./scripts/log.sh

set -euo pipefail

on_interrupt() {
    git remote remove wini-template
}

trap on_interrupt SIGINT TERM EXIT


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

remote_template_hashes="$(git log "wini-template/$branch" --pretty=format:"%H")"
current_repo_hashes="$(git log --pretty=format:"%H")"

while IFS= read -r remote_hash; do
    while IFS= read -r current_hash; do
        if [[ $current_hash == $remote_hash ]]; then
            if [[ $last_commit_hash == $remote_hash ]]; then
                info 'Up to date!'
            else
                sed -E -i "s/last_commit_hash(\s*)=(\s*)(.*)/last_commit_hash\1=\2\"$remote_hash\"/g" wini.toml
                info 'Successfully updated `last_commit_hash` to '$remote_hash
            fi

            git remote remove wini-template
            exit 0
        fi
    done <<< "$current_repo_hashes"
done <<< "$remote_template_hashes"

git remote remove wini-template
