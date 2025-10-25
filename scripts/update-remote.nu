source ./check.nu

def main [] {
    let branches_and_paths = open ./scripts/branches.json | transpose name features | each {|i| 
        print $"# -------------------------------------- #"
        print $"# --- Testing feature \"($i.name)\" --- #"
        print $"# -------------------------------------- #"
        let path = check-with-features ($i.features)
        { "path": $path, "branch": $i.name }
    }

    let wini_template_dir = mktemp -d
    cd $wini_template_dir

    git clone https://github.com/wini-rs/wini-template

    cd wini-template

    for branch_and_path in $branches_and_paths {
        let temporary_branch = $"($branch_and_path.branch)-(random uuid)"
        git checkout $branch_and_path.branch
        git pull
        git checkout -b $temporary_branch
        rm -rf *

        let basename = ($"($branch_and_path.path)" | path basename)
        mv $branch_and_path.path $"($wini_template_dir)/wini-template"
        cd $basename
        mv * ..
        cd ..
        rm -rf basename

        # diff -ru --exclude=.git $branch_and_path.path $"($wini_template_dir)/wini-template" | delta

        gitui

        git push --set-upstream origin $temporary_branch
    }
}
