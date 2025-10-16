use {proc_macro::Span, std::path::PathBuf};

pub fn get_current_file_path() -> Option<PathBuf> {
    Span::call_site().local_file()
}

/// Get javascript and css files in the directory of the proc_macro
pub fn get_js_or_css_files_in_current_dir() -> Vec<String> {
    let Some(file_path) = get_current_file_path() else {
        return Vec::new();
    };

    let Some(dirname) = file_path.parent() else {
        return Vec::new();
    };

    let mut files = Vec::new();

    if let Ok(readir) = std::fs::read_dir(dirname) {
        for entry in readir {
            let entry = entry.unwrap();
            let path = entry.path();

            // Check if the path is a file and ends with .css
            if path.is_file() && path.extension().is_some_and(|s| s == "js" || s == "css") {
                files.push(path.to_string_lossy().into_owned());
            }
        }
    }

    files
}
