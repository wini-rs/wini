use std/assert

source ../check.nu

def main [] {
    let dir = (check-with-features ["ssg" "nushell"])
    cd $dir

    # Testing build
    nix develop -c nu -c "just build-prod"

    cd -

    assert (
        (cat ./scripts/tests/expected-index.html | str trim | split row '<' | sort) == (cat $"($dir)/dist/index.html" | split row '<' | sort)
    )
}

