use proc_macro::Span;


/// Get javascript and css files in the directory of the proc_macro
pub fn get_js_or_css_files_in_current_dir() -> Vec<String> {
    let span = Span::call_site();
    let source = span.source_file();
    let simple_path = source.path();

    let mut path_elements = simple_path
        .to_str()
        .unwrap()
        .split('/')
        .collect::<Vec<&str>>();

    path_elements.pop();

    let path = path_elements.join("/");

    let mut files = Vec::new();

    if let Ok(readir) = std::fs::read_dir(path) {
        for entry in readir {
            let entry = entry.unwrap();
            let path = entry.path();

            // Check if the path is a file and ends with .css
            if path.is_file() {
                if path
                    .extension()
                    .map(|s| s == "js" || s == "css")
                    .unwrap_or(false)
                {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    files
}
