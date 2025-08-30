# Ask the path for the package bundles

source ./utils.nu


# Helper function to show error for invalid paths
def show_invalid_path [file, node_modules_pkg] {
    print "=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
    error $"Invalid path: `($node_modules_pkg)/($file)`"
    info $"Files in ($node_modules_pkg):"
    print (ls ...(glob $"($node_modules_pkg)/**/*.{js,css}" | into glob))
    print "=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
}


def main [pkg] {
    let pkg_files = (open ./packages-files.toml | get -o $pkg)

    if $pkg_files != null {
        error $"($pkg) is already installed"
        exit 1
    }

    let node_modules_pkg = $"./node_modules/($pkg)";
    mut default_file: any = null;
    mut default_file_prompt: any = null;

    if ($"($node_modules_pkg)/dist" | path exists) {
        let guess_default_file = (du ($"($node_modules_pkg)/dist/*.js" | into glob) | sort-by apparent | get -o 0)
        if $guess_default_file != null {
            $default_file = "./dist" | path join ($guess_default_file | get "path" | path basename)
            $default_file_prompt = $"\(Default: \e[90m($default_file)\e[0m\)"
        }
    } else {
        error $"Didn't find any `dist` directory in `($node_modules_pkg)`:"
        ls $node_modules_pkg
    }

    mut path_of_file = '';

    while ($path_of_file == '') {
        info $"The path\(s\) that you will enter are relatives to ($node_modules_pkg)"
        info "You can specify multiple files by separating them with spaces."
        ask $"Path of the new file to distribute ? ($default_file_prompt) "
        $path_of_file = input

        if ($path_of_file == '') and ($default_file != null) {
            $path_of_file = $default_file
            info $"Going to use the default file: ($default_file)"
        } else if ($"($node_modules_pkg)/($path_of_file)" | path exists) {
            info $"Going to use: ($path_of_file)"
        } else if ($path_of_file | str contains ' ') {
            let has_any_invalid_file = (
                $path_of_file
                | split row ' '
                | any { |file|
                    if ($"($node_modules_pkg)/($file)" | path exists | neg) {
                        show_invalid_path $file $node_modules_pkg
                        true
                    } else {
                        false
                    }
                }
            )

            if $has_any_invalid_file {
                $path_of_file = ''
            }
        } else {
            show_invalid_path $path_of_file $node_modules_pkg
            $path_of_file = ''
        }
    }

    if ($path_of_file | str contains ' ') {
        let arr = ($path_of_file | str replace ' ' '", "');
        $'"($pkg)" = ["($arr)"]' | save -a ./packages-files.toml
    } else {
        $'"($pkg)" = "($path_of_file)"' | save -a ./packages-files.toml
    }
}
