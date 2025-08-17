# This script was only used once to initialize all the branches
# You should use `./update-remote.nu` now

source ./check.nu

def main [] {
    let branches_and_paths = open ./scripts/branches.json | transpose name features | each {|i| 
        print $"# -------------------------------------- #"
        print $"# --- Testing feature \"($i.name)\" --- #"
        print $"# -------------------------------------- #"
        let path = check-with-features $i.features

        { "path": $path, "branch": $i.name }
    }

    let wini_template_dir = mktemp -d
    cd $wini_template_dir

    git clone git@github.com:wini-rs/wini-template.git

    cd wini-template

    for branch_and_path in $branches_and_paths {
        git checkout main
        git checkout -b $branch_and_path.branch


        diff -ru --exclude=target --exclude=.git $"($wini_template_dir)/wini-template" $branch_and_path.path | delta

        rm -rf *
        cp -r $"($branch_and_path.path)/.gitignore" $"($wini_template_dir)/wini-template"
        cp -r $"($branch_and_path.path)/.env" $"($wini_template_dir)/wini-template"
        cp -r $branch_and_path.path $"($wini_template_dir)/wini-template"
        cd $"($wini_template_dir)/wini-template/wini"
        mv * ..
        cd ..
        rm -rf wini

        rm -rf (dirname $branch_and_path.path)

        git add .
        git commit -m $"Initial version of ($branch_and_path.branch)"
        let resp = input "Ok to push ? [y/N] " | str downcase

        if ($resp | str contains 'y') {
            git push -u origin $branch_and_path.branch
        } else {
            print "EXIT!"
            exit 1
        }
    }
}
