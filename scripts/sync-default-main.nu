let temp_dir = mktemp -d

cd $temp_dir

git clone git@github.com:wini-rs/wini-template
cd wini-template

git checkout main
git merge origin/default
git push -u origin main
