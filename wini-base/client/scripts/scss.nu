# This script is in charge of compiling scss files to css.

source ./utils.nu

fd -e scss . src | lines | each { |file|
    sass $"($file):($file | replace-ext 'scss' 'css')" --no-source-map --style compressed
}
