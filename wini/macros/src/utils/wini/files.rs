use proc_macro::Span;


/// Get javascript and css files in the directory of the proc_macro
pub fn get_js_or_css_files_in_current_dir() -> Vec<String> {
    let span = Span::call_site();

    let Some(maybe_file) = span.local_file() else {
        return vec![];
    };
    let Some(dirname) = maybe_file.parent() else {
        return vec![];
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
