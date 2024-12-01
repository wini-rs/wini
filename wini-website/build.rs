use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("public").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().to_str().unwrap());
        }
    }
}
