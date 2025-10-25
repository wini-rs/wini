source ./check.nu

def main [] {
    let branches_and_paths = open ./scripts/branches.json | transpose name features | each {|i| 
        print $"# -------------------------------------- #"
        print $"# --- Testing feature \"($i.name)\" --- #"
        print $"# -------------------------------------- #"
        let path = check-with-features ($i.features | append "test")
        { "path": $path, "branch": $i.name }
    }

    let wini_template_dir = mktemp -d
    cd $wini_template_dir

    git clone https://github.com/wini-rs/wini-template

    cd wini-template

    for branch_and_path in $branches_and_paths {
        git checkout $branch_and_path.branch
        git checkout -b $"($branch_and_path.branch)-(random uuid)"

        rm -rf *

        cp -r $branch_and_path.path $"($wini_template_dir)/wini-template"
        diff -ru --exclude=.git $branch_and_path.path $"($wini_template_dir)/wini-template" | delta

        git push
        rm -rf $branch_and_path.path
    }

    
}
