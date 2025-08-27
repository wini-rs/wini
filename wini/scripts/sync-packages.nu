# This script will update the modules in the public directory with the files specified in `./packages-files.toml`

source ./utils.nu

let path = (open ./wini.toml | get "path")
let public_path = ($path | get "public" | path parse)
let modules_path = ($path | get "modules" | path parse)

let relative_modules_path = './src/' | path join $public_path | path join $modules_path;

# Remove everything in the modules directory
try {
    rm -r ($"($relative_modules_path)/*" | into glob)
} catch {
    info "Continuing..."
}

# Sync node_modules with modules
open ./packages-files.toml | get "keys" | items { |key, value|
    if ($"./node_modules/($key)" | path exists | neg) {
        error "$key is not installed!!!"
        info "File(s) of $key not copied."
    } else {
        mkdir -p $"($relative_modules_path)/($key)"

        # Multiple files vs one file
        if ($value | describe | str contains 'list') { 
            $value | each { |file|
                try {
                    cp $"./node_modules/($key)/($file)" $"($relative_modules_path)/($key)"
                } catch {
                    error $"Package ($key) doesn't have the file ($file)"
                }
            }
        } else {
            try {
                cp $"./node_modules/($key)/($value)" $"($relative_modules_path)/($key)"
            } catch {
                error $"Package ($key) doesn't have the file ($value)"
            }
        }
    }
}
