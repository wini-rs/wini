use {
    crate::shared::wini::config::TomlLoadingError,
    serde::Deserialize,
    std::{io, path::Path},
    walkdir::WalkDir,
};

/// This function will try to get all the files in a directory, including subdirectories and return
/// their relative paths.
///
/// # Example
///
/// ```ignore
/// ├── a
/// ├── b/
/// │   └── d
/// ├── c
/// └── d/
/// ```
///
/// Will result in
///
/// `["a", "b/d", "c"]`
pub fn get_files_in_directory<P: AsRef<Path>>(dir: P) -> std::io::Result<Vec<String>> {
    let mut files = Vec::new();

    // Read the directory
    for entry in std::fs::read_dir(dir.as_ref())? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file() {
            // Get the file name and its path
            if path.file_name().and_then(|n| n.to_str()).is_some() {
                files.push(path.to_string_lossy().replace("./public", ""));
            }
        } else if path.is_dir() {
            files.extend(get_files_in_directory(path)?);
        }
    }

    Ok(files)
}

/// This function will try to get all the files in a directory, including subdirectories with a
/// particular extension (.css, .js) and return their relative paths.
///
/// # Example
///
/// ```ignore
/// ├── a.js
/// ├── a_not_js
/// ├── b/
/// │   └── d.css
/// ├── c
/// ├── d/
/// └── e.css
/// ```
///
/// Searching extensions `["js", "css"]`
///
/// Will result in
///
/// `["a.js", "b/d.css", "e.css"]`
pub fn get_files_in_directory_per_extensions(dir: &str, extensions: &[&str]) -> Vec<String> {
    let extensions_with_dots = extensions
        .iter()
        .map(|ext| format!(".{ext}"))
        .collect::<Vec<String>>();

    WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| {
            entry.ok().and_then(|file| {
                extensions_with_dots
                    .iter()
                    .any(|ext| file.path().to_str().is_some_and(|s| s.ends_with(ext)))
                    .then(|| {
                        file.path()
                            .to_str()
                            .expect("Already verified before")
                            .to_string()
                    })
            })
        })
        .collect::<Vec<_>>()
}

pub fn toml_from_path_as_static_str<T>(path: &'static str) -> Result<T, TomlLoadingError>
where
    T: for<'de> Deserialize<'de>,
{
    toml::from_str(
        std::fs::read_to_string(path)
            .map_err(|err| {
                match err.kind() {
                    io::ErrorKind::NotFound => TomlLoadingError::ConfigFileDoesntExists(path),
                    _ => TomlLoadingError::OtherIo(err),
                }
            })?
            .as_ref(),
    )
    .map_err(|err| TomlLoadingError::InvalidToml(err, path))
}
