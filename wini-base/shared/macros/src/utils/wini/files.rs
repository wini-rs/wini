use {proc_macro::Span, std::path::PathBuf};

pub fn get_current_file_path() -> Option<PathBuf> {
    Span::call_site().local_file()
}

type StringWithLeadingSlash = String;

/// Get javascript and css files in the directory of the proc_macro
///
/// # Return type
/// The return type in `Vec<String>` and not `HashSet<String>` because the result type will only be
/// used in `quote!()` macros
///
/// # Example
///
/// ```text
/// ├── a.css
/// ├── b/
/// │   └── d.js
/// ├── c.js
/// └── d/
/// ```
///
/// Will result in
///
/// `["/a.css", "/c.js"]`
///
/// # Panic
/// This function can panic.
/// This behavior is acceptable since it will only be executed at compile-time.
pub fn get_js_or_css_files_in_current_dir() -> Vec<StringWithLeadingSlash> {
    let Some(file_path) = get_current_file_path() else {
        return Default::default();
    };

    let Some(dirname) = file_path.parent() else {
        return Default::default();
    };

    let mut files = Vec::default();

    if let Ok(readir) = std::fs::read_dir(dirname) {
        for entry in readir {
            let entry = entry.unwrap();
            let path = entry.path();

            // Check if the path is a file and ends with .css
            if path.is_file() && path.extension().is_some_and(|s| s == "js" || s == "css") {
                files.push(format!("/{}", path.display()));
            }
        }
    }

    files
}
