# This script is in charge of compiling scss files to css.

fd -e scss . src | lines | each { |file|
    sass $"($file):($file | sed 's/\.scss$/.css/g')" --no-source-map --style compressed
}
