let tmp_dir = mktemp -d

cd $tmp_dir

git clone git@github.com:wini-rs/wini-template
chmod -R -w wini-template
cd wini-template

print $"cd ($tmp_dir)/wini-template"
