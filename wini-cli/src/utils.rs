use {rand::Rng, std::path::Path};

/// Creates a random string of length `length`.
pub fn generate_random_string(length: usize) -> String {
    let rng = rand::rng();
    rng.sample_iter(rand::distr::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Copy recursively a directory
pub fn copy_dir_all<Src: AsRef<Path>, Dst: AsRef<Path>>(src: Src, dst: Dst) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
