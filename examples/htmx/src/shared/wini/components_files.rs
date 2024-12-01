use {
    super::{
        config::SERVER_CONFIG,
        dependencies::normalize_relative_path,
        err::ExitWithMessageIfErr,
    },
    crate::{concat_paths, utils::wini::file::get_files_in_directory_per_extensions},
    regex::Regex,
    std::{collections::HashMap, sync::LazyLock},
};

/// Files linked to a components
#[derive(Debug)]
pub struct ComponentsFiles {
    pub js: HashMap<String, Vec<String>>,
    pub css: HashMap<String, Vec<String>>,
}

/// Get all the files linked to a component
pub static COMPONENTS_FILES: LazyLock<ComponentsFiles> = LazyLock::new(|| {
    let mut js_hm = HashMap::new();
    let mut css_hm = HashMap::new();

    let components_path = format!("src/{}", SERVER_CONFIG.path.components);
    let regex_to_remove_components_dir =
        Regex::new(&format!("^/{components_path}/")).exit_with_msg_if_err("Expected a valid regex");

    for file_path in get_files_in_directory_per_extensions(&components_path, &["js", "css"]) {
        let path_to_file_str_non_normalized = regex_to_remove_components_dir
            .replace(&file_path, "")
            .to_string();

        let path_to_push = normalize_relative_path(concat_paths!(
            &components_path,
            &path_to_file_str_non_normalized
        ))
        .display()
        .to_string();


        if path_to_file_str_non_normalized.ends_with(".css") {
            css_hm
                .entry(path_to_file_str_non_normalized)
                .or_insert_with(Vec::new)
                .push(path_to_push);
        } else {
            js_hm
                .entry(path_to_file_str_non_normalized)
                .or_insert_with(Vec::new)
                .push(path_to_push);
        }
    }

    ComponentsFiles {
        js: js_hm,
        css: css_hm,
    }
});
