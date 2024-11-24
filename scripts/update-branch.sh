#!/usr/bin/env bash

# Update a template branch from a commit


source_branch="$1"
target_branch="$2"
commit_hash="$3"

git checkout $target_branch

git cherry-pick $commit_hash

# Handle any conflicts that may arise
if [ $? -ne 0 ]; then
    echo "Conflicts detected. Please resolve the conflicts and continue the cherry-pick."
    exit 1
fi

git push

git checkout -
