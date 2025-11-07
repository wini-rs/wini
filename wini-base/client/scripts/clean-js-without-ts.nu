# Clean javascript files that don't have an associated typescript file in `./src`

source ./utils.nu

fd -e js . ./src -HI |  lines | each { |js_file|
    if not ($js_file | replace-ext 'js' 'ts' | path exists) {
        rm $js_file
        print $"Removed ($js_file)"
    }
}
