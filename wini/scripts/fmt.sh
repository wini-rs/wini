#!/usr/bin/env bash

# This script is in charge of formatting files

source ./scripts/yesno.sh

set -euo pipefail


bunx biome check --write
cargo +nightly fmt
cargo clippy --fix || {
    read -p "Directory is dirty. Do you still want to run \`cargo clippy --fix\` ? $(yesno n) " yn

    if [[ $yn =~ yY ]]; then 
        cargo clippy --fix --allow-dirty
    fi
}
