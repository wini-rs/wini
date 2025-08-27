# This script will update the modules in the public directory with the files specified in `./packages-files.toml`

source ./utils.nu

let path = (open ./wini.toml | get "path")
let public_path = ($path | get "public" | path parse)
let modules_path = ($path | get "modules" | path parse)

let relative_modules_path = './src/' | path join $public_path | path join $modules_path;

# Remove everything in the modules directory
rm -r "${relative_modules_path:?}"/* || info "Continuing..."


keys=$(yq -p toml "keys" < ./packages-files.toml | yq '.[]')

for key in $keys; do
    if [ ! -d "node_modules/$key" ]; then
        error "$key is not installed!!!"
        info "File(s) of $key not copied."
        continue
    fi


    mkdir -p "$relative_modules_path/$key"

    key_type="$(yq -p toml ".\"$key\" | type" < ./packages-files.toml)"


    # Multiple files vs one file
    # NOTE: In yq, array has type "!!seq"
    # All types: `yq 'map(type)' <<< '[0, false, ["aa", "b"], {}, null, "hello"]'`
    if [ "$key_type" = '!!seq' ]; then
        for value in $(yq -r ".\"$key\"[]" ./packages-files.toml); do
            cp "./node_modules/$key/$value" "$relative_modules_path/$key" || error "Package $key doesn't have the file $value"
        done
    else
        value=$(yq -p toml ".\"$key\"" < ./packages-files.toml)
        cp "./node_modules/$key/$value" "$relative_modules_path/$key" || error "Package $key doesn't have the file $value"
    fi
done

