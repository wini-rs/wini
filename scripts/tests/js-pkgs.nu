use std/assert

source ../check.nu

def main [] {
    let dir = (check-with-features ["ssg" "posix-sh"])
    cd $dir

    yes '' | just js-add "alpinejs"

    sed -i 's/#\[page\]/#[page(js_pkgs = ["alpinejs"])]/g' ./src/pages/hello/mod.rs

    # Testing build
    nix develop -c nu -c "just build-prod"

    cd -

    assert (
        (cat $"($dir)/dist/index.html" | str contains 'modules/alpinejs')
    )
}

