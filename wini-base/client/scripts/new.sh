#!/usr/bin/env bash

# This script is in charge of creating file with the `just new` command

source ./scripts/log.sh
source ./scripts/yesno.sh

set -euo pipefail

# Function to get the kind of file to create
get_kind_new() {
    case "$1" in
        'layout'|'l')
            echo 'layout'
            ;;
        'component'|'c')
            echo 'component'
            ;;
        *)
            echo 'page'
            ;;
    esac
}


get_directory_of_kind_new() {
    case "$1" in 
        'layout')
            yq ".path.layouts" ./wini.toml
            ;;
        'component')
            yq ".path.components" ./wini.toml
            ;;
        'page')
            yq ".path.pages" ./wini.toml
            ;;
    esac 
}


kind=$(get_kind_new "$1")
directory_of_kind_new="$(get_directory_of_kind_new "$kind")"
src_directory_of_kind_new="./src/${directory_of_kind_new#./}"



info "Going to create a new '$kind' from template"
ask "Which path should it be located at: " 
read -r path

relative_path="$src_directory_of_kind_new/$path"
# Check if this already exists.
[ -e "$relative_path" ] && { error "Already exists."; exit 1; }


ask "Create a new page at '\e[1m$relative_path\e[0m' ? $(yesno y) "
read -r yn

if [ "$yn" = 'N' ] || [ "$yn" = 'n' ]; then
    error "Aborting." 
    exit 1
fi
mkdir -p "$relative_path"
cp -r ./scripts/templates/"$kind"/* "$relative_path"
info "Created '\e[1m$path\e[0m'."

while [ "$relative_path" != "$src_directory_of_kind_new" ]; do
    basename="$(basename "$relative_path")"
    relative_path="$(dirname "$relative_path")"
    if [ -e "$relative_path/mod.rs" ]; then
        echo "pub mod $basename;" >> "$relative_path/mod.rs"
    else
        echo "pub mod $basename;" > "$relative_path/mod.rs"
    fi
done
