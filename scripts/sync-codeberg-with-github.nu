let temp_dir = mktemp -d

print $"Temp directory: ($temp_dir)"

cd $temp_dir

try {
    git clone git@github.com:wini-rs/wini-template
} catch {
    git clone https://github.com/wini-rs/wini-template
}
cd wini-template

git push https://codeberg.org/wini/wini-template --mirror

cd -

rm -rf $temp_dir
