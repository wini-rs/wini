# This script is in charge of compiling typescript files to javascript.

source ./utils.nu;

fd -e ts . src | lines | each { |file|
    let js_filename = ($file | replace-ext 'ts' 'js')
    bun build --no-bundle $file -e '*' --minify-syntax --minify-whitespace o> $js_filename
    sed -i r#'s/import\s*\([A-Za-z0-9_\*]*\)\?\s*,\?\s*\({[^}]*}\)\?\s*\(from\)\?\s*\(['"][^'"]*['"]\);\?//g'# $js_filename
}

bun build --no-bundle "./public/helpers.js" -e '*' --minify-syntax --minify-whitespace o> ./public/helpers.min.js
