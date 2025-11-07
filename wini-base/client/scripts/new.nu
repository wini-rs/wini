# This script is in charge of creating file with the `just new` command

source ./utils.nu


def main [$raw_kind_str: string] {
    let kind = (
        match $raw_kind_str {
            'layout' | 'l' => 'layout',
            'component' | 'c' => 'component',
            'page' | 'p' | _ => 'page'
        }
    )
    let directory_of_kind_new = (open ./wini.toml | get "path" | get $'($kind)s')
    let src_directory_of_kind_new = ("./src/" | path join $directory_of_kind_new)


    info $"Going to create a new '($kind)' from template"
    ask "Which path should it be located at: " 
    let path = input
    mut relative_path = ($src_directory_of_kind_new | path join $path)

    # Check if this already exists.
    if ($relative_path | path exists) {
        error "Already exists.";
        exit 1;
    }

    let yn = prompt_yesno $"Create a new page at '\e[1m($relative_path)\e[0m' ?" 'y'

    if not $yn {
        error "Aborting." 
        exit 1
    }

    mkdir $relative_path
    cp -r ($"./scripts/templates/($kind)/*" | into glob) $relative_path
    info $"Created '\e[1m($path)\e[0m'."

    while $relative_path != $src_directory_of_kind_new {
        let basename = ($relative_path | path basename)
        $relative_path = ($relative_path | path dirname)

        $"pub mod ($basename);" | save -a $"($relative_path)/mod.rs"
    }
}
