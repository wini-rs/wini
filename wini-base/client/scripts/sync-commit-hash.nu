# This script is in charge of syncing the `last_commit_hash` of `wini.toml`

source ./utils.nu

let origin = (open ./wini.toml | get "origin")

let wini_toml_last_commit_hash = ($origin | get "last_commit_hash")
let remote_url = ($origin | get "remote_url")
let branch = ($origin | get "branch")

if $remote_url == 'NONE' {
    error "No remote repository. The project was created using a local repository."
    exit 1
}

try {
    try { git remote remove wini-template }
    git remote add wini-template $remote_url
    git fetch wini-template

    let remote_template_hashes = (git log $"wini-template/($branch)" --pretty=format:"%H")
    let current_repo_hashes = (git log --pretty=format:"%H")

    $remote_template_hashes | lines | each { |remote_hash|
        $current_repo_hashes | lines | each { |current_hash|
            if $current_hash == $remote_hash {
                if $wini_toml_last_commit_hash == $remote_hash {
                    info 'Up to date!'
                } else {
                    open ./wini.toml
                    | update origin.last_commit_hash $remote_hash
                    | to toml
                    | save -f wini.toml
                    taplo format wini.toml
                    info $'Successfully updated `last_commit_hash` to ($remote_hash)'
                }

                git remote remove wini-template
                exit 0
            }
        }
    }

    git remote remove wini-template
} catch {
    git remote remove wini-template
}
