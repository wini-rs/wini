#!/usr/bin/env bash

# Ask the path for the package bundles

source ./scripts/log.sh


all_valid_files() {
    for path in "$@"; do
        [ -f "$path" ] || return 1
    done
    return 0
}

# Helper function to show error for invalid paths
show_invalid_path() {
    echo "=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
    error "Invalid path: \`$2/$1\`"
    info "Files in $2:"
    ls -1F "$2"
    echo "=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
}


PKG="$1"

if [ -n "$(yq -p toml ".$PKG" < ./packages-files.toml)" ]; then
    error "$PKG is already installed"
    exit 1
fi


public_path=$(yq ".path.public" ./wini.toml)
modules_path=$(yq ".path.modules" ./wini.toml)
node_modules_pkg="./node_modules/$PKG" 

default_file=


if [ -d "$node_modules_pkg/dist" ]; then
    default_file=$(du -b "$node_modules_pkg/dist"/*.js 2>/dev/null | sort -n | head -n 1 | cut -f2)
    if [ -n "$default_file" ]; then
        default_file=".${default_file//$node_modules_pkg/}"
        default_file_prompt="(Default: \e[90m$default_file\e[0m)"
    fi
else
    error "Didn't find any \`dist\` directory in \`$node_modules_pkg\`:"
    ls -l1F "$node_modules_pkg"
fi


while [ -z "$path_of_file" ]; do
    info "The path(s) that you will enter are relatives to $node_modules_pkg"
    info "You can specify multiple files by separating them with spaces."
    ask "Path of the new file to distribute ? $default_file_prompt "
    read -r path_of_file

    if [ -z "$path_of_file" ] && [ -n "$default_file" ]; then
        path_of_file="$default_file"
        info "Going to use the default file: $default_file"
    elif [ -f "$node_modules_pkg/$path_of_file" ]; then
        info "Going to use: $path_of_file"
    elif grep -q ' ' <<< "$path_of_file"; then
        for file in $path_of_file; do
            if [ ! -f "$node_modules_pkg/$file" ]; then
                show_invalid_path "$file" "$node_modules_pkg"
                path_of_file=
                continue
            fi
        done
    else
        show_invalid_path "$path_of_file" "$node_modules_pkg"
        path_of_file=
    fi
done

if grep -q ' ' <<< "$path_of_file"; then
    echo "$PKG = [\"${path_of_file/ /\", \"}\"]" >> ./packages-files.toml
else
    echo "$PKG = \"$path_of_file\"" >> ./packages-files.toml
fi
