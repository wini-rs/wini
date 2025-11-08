use {
    crate::shared::wini::config::TomlLoadingError,
    serde::Deserialize,
    std::{collections::HashSet, ffi::OsStr, io, path::Path},
    walkdir::WalkDir,
};

type StringWithLeadingSlash = String;

/// This function will try to get all the files in a directory, including subdirectories and return
/// their relative paths.
///
/// # Example
///
/// ```text
/// ├── a
/// ├── b/
/// │   └── d
/// ├── c
/// └── d/
/// ```
///
/// Will result in
///
/// `["/a", "/b/d", "/c"]`
pub fn get_files_in_directory(dir: impl AsRef<Path>) -> HashSet<StringWithLeadingSlash> {
    WalkDir::new(&dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| {
            match entry {
                Ok(entry) if entry.file_type().is_dir() => None,
                Ok(entry) => {
                    let entry_path = entry.into_path();
                    Some({
                        format!(
                            "/{}",
                            entry_path
                                .strip_prefix(&dir)
                                .unwrap_or(&entry_path)
                                .display()
                        )
                    })
                },
                Err(err) => {
                    log::warn!("Error reading an entry: {err:#?}");
                    None
                },
            }
        })
        .collect()
}

/// This function will try to get all the files in a directory, including subdirectories with a
/// particular extension (.css, .js) and return their relative paths.
///
/// # Example
///
/// ```text
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
/// `["/a.js", "/b/d.css", "/e.css"]`
pub fn get_files_in_directory_per_extensions(
    dir: impl AsRef<Path>,
    extensions: &[&OsStr],
    with_strip: bool,
) -> HashSet<StringWithLeadingSlash> {
    WalkDir::new(&dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| {
            match entry {
                Ok(entry)
                    if extensions
                        .iter()
                        .any(|ext| entry.path().extension() == Some(ext)) =>
                {
                    let entry_path = entry.into_path();
                    Some({
                        format!(
                            "/{}",
                            if with_strip {
                                entry_path
                                    .strip_prefix(&dir)
                                    .unwrap_or(&entry_path)
                                    .display()
                            } else {
                                entry_path.display()
                            }
                        )
                    })
                },
                Ok(_entry) => None,
                Err(err) => {
                    log::warn!("Error reading an entry: {err:#?}");
                    None
                },
            }
        })
        .collect()
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
