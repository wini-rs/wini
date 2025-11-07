source ./utils.nu

if (git status | str contains 'nothing to commit, working tree clean' | neg) {
    error "Commit your current changes before updating wini"
    exit 1
}

let origin = (open ./wini.toml | get "origin")

let last_commit_hash = ($origin | get "last_commit_hash")
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

    # Check for coherent last_commit_hash
    if (git merge-base --is-ancestor $last_commit_hash $"wini-template/($branch)" | neg) {
        error $"Invalid `last_commit_hash`: doesn't exist in remote wini-template/($branch)"
        git remote remove wini-template
        exit 1
    }

    if (git cherry-pick $"($last_commit_hash)..wini-template/($branch)" | neg) {
        try {
            git remote remove wini-template
        }
        error 'Cherry-pick failed; please resolve conflicts and after that, do `wini sync-commit-hash`'
        exit 1
    }

    try { git remote remove wini-template }

    just sync-commit-hash
} catch {
    git remote remove wini-template
}
