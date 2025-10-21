use std/assert

source ../check.nu

def main [] {
    # First test: only with one occurrence
    let dir = (check-with-features ["ssg" "posix-sh"])
    cd $dir

    nix develop -c nu -c "yes '' | just js-add 'alpinejs'"

    sed -i 's/#\[page\]/#[page(js_pkgs = ["alpinejs"])]/g' ./src/pages/hello/mod.rs

    # Testing build
    nix develop -c nu -c "just build-prod"

    cd -

    assert (
        (cat $"($dir)/dist/index.html" | str contains 'modules/alpinejs')
    )

    rm -r $dir

    # ---

    # Second test: with multiple occurrences
    let dir = (check-with-features ["ssg" "posix-sh"])
    cd $dir

    nix develop -c nu -c "yes '' | just js-add 'alpinejs'"

    sed -i 's/#\[page\]/#[page(js_pkgs = ["alpinejs"])]/g' ./src/pages/hello/mod.rs
    sed -i 's/#\[layout\]/#[layout(js_pkgs = ["alpinejs"])]/g' ./src/layouts/header/mod.rs

    # Testing build
    nix develop -c nu -c "just build-prod"

    cd -

    assert (
        (cat $"($dir)/dist/index.html" | str contains 'modules/alpinejs')
    )
    assert (
        (cat $"($dir)/dist/index.html" | str index-of 'modules/alpinejs') ==
        (cat $"($dir)/dist/index.html" | str index-of --end 'modules/alpinejs')
    )

    rm -r $dir
}

