source ./check.nu

def main [] {
    let branches_and_paths = []

    open ./scripts/branches.json | transpose name features | each {|i| 
        print $"# -------------------------------------- #"
        print $"# --- Testing feature \"($i.name)\" --- #"
        print $"# -------------------------------------- #"
        print $"($i.features | append "test")"
        let path = check-with-features ($i.features | append "test")
        $branches_and_paths = ($branches_and_paths | append { "path": $path, "branch": $i.name })
    }

    let wini_template_dir = mktemp -d
    cd $wini_template_dir

    git clone https://github.com/wini-rs/wini-template

    cd wini-template

    for branch_and_path in $branches_and_paths {
        git checkout $branch_and_path.branch
        git checkout -b $"($branches_and_paths.branch)-(random uuid)"

        rm -rf *

        cp -r $branch_and_path.path $wini_template_dir
        diff -ru --exclude=.git $branch_and_path.path $wini_template_dir | delta
    }
}
