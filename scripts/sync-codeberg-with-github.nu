let temp_dir = mktemp -d

print $"Temp directory: ($temp_dir)"

cd $temp_dir

git clone git@github.com:wini-rs/wini-template
cd wini-template

git remote set-url origin https://codeberg.org/wini/wini-template
git pull
git push

cd -

rm -rf "$temp_dir"
