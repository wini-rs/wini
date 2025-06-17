use {
    super::{
        JS_FILES,
        err::ExitWithMessageIfErr,
        tsconfig::{TSCONFIG_PATHS, TsConfigPathsPrefix},
    },
    crate::concat_paths,
    regex::Regex,
    std::{
        collections::HashMap,
        path::{Component, Path, PathBuf},
        sync::LazyLock,
    },
};

pub static REGEX_DEPENDENCY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(import|from)\s*["']([^'"]+)["'](;|\n)"#)
        .expect("This should always be a valid regex.")
});

pub static REGEX_IS_PACKAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z_0-9]").expect("This should always be a valid regex."));

pub static SCRIPTS_DEPENDENCIES: LazyLock<HashMap<String, Option<Vec<String>>>> =
    LazyLock::new(|| {
        LazyLock::force(&REGEX_IS_PACKAGE);

        JS_FILES
            .keys()
            .map(|script| (script.to_owned(), script_dependencies(script)))
            .collect()
    });


/// Normalizes a relative file path by resolving `.` (current directory) and `..` (parent directory) components.
///
/// # Example
///
/// ```rs
/// use PROJECT_NAME_TO_RESOLVE::shared::wini::dependencies::normalize_relative_path;
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new("./folder/../file.txt");
/// let normalized = normalize_relative_path(path);
/// assert_eq!(normalized, PathBuf::from("file.txt"));
/// ```
pub fn normalize_relative_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut components = Vec::new();

    for component in path.as_ref().components() {
        match component {
            Component::CurDir => {
                // Ignore "./" (current directory)
            },
            Component::ParentDir => {
                // Remove the last component if possible, but only if it's not a root or a prefix
                if let Some(last) = components.last() {
                    if *last == Component::ParentDir {
                        components.push(component);
                    } else {
                        components.pop();
                    }
                } else {
                    components.push(component); // If it's at the start, keep it
                }
            },
            _ => {
                components.push(component);
            },
        }
    }

    let mut normalized_path = PathBuf::new();
    for component in components {
        normalized_path.push(component.as_os_str());
    }

    normalized_path
}


/// Get the dependencies of a JavaScript file.
///
/// This only gets the files or packages that a file needs.
/// If a package depend on other packages, they will not be included.
/// This is used to easily import <script/>s in head
///
/// # Example:
/// `file1.js` // require("./file2")
/// `file2.js` // require("debug")
///
/// Will produce the following slice:
/// `["file2.js", "debug"]`.
///
/// This should be converted to
///
/// ```html
/// <head>
///     ...
///     <script src="path/to/debug.min.js"></script>
///     <script src="file2.js"></script>
///     ...
/// </head>
///
/// # Panic
///
/// If there is an error finding a dependency
fn script_dependencies(path: &str) -> Option<Vec<String>> {
    let mut path_str = path.strip_prefix("/").unwrap_or(path).replace(".js", ".ts");
    let mut path = std::path::Path::new(&path_str);

    if !path.exists() {
        path_str = path_str.replace(".ts", ".js");
        path = std::path::Path::new(&path_str);
    }

    let contents = std::fs::read_to_string(path).exit_with_msg_if_err("IO Error");

    let caps = REGEX_DEPENDENCY.captures_iter(&contents);

    let dependencies = caps
        .into_iter()
        .map(|m| m.extract::<3>())
        .map(|ex| ex.1[1].to_string())
        .collect::<Vec<String>>();

    if dependencies.is_empty() {
        None
    } else {
        let mut relatives_dependencies = vec![];

        // Ok to clone since this function will only be called on initialization of LazyLock
        for dep in dependencies {
            let is_dep_package = REGEX_IS_PACKAGE.is_match(&dep);

            // We need to convert the dependency to it's correct path

            // If an import starts with a ".", it's a path to a file. In this case, we want to
            // have it's path relative to the file it's referenced from.
            let dep_relative_path = if dep.starts_with('.') {
                let dep = concat_paths!(
                    path.parent().expect("Path should have a parent."),
                    if dep.ends_with(".js") {
                        dep
                    } else {
                        dep + ".ts"
                    }
                )
                .to_str()
                .expect("Not empty.")
                .to_string();

                normalize_relative_path(dep).display().to_string()
            }
            // Resolve tsconfig paths. <=> If it's a file that needs to be resolved with
            // `tsconfig.compilerOptions.paths`.
            else if let Some(prefix_path) = TSCONFIG_PATHS
                .prefixes()
                .iter()
                .find(|prefix| dep.starts_with(*prefix))
            {
                let vec = TSCONFIG_PATHS
                    .get(*prefix_path)
                    .expect("Already matched the key");

                // If there is only one path to resolve, we know which one it is! (the first)
                if vec.len() == 1 {
                    concat_paths!(&vec[0], &dep[prefix_path.len()..])
                        .display()
                        .to_string()
                } else {
                    let mut resolved_path = None;

                    for path in vec {
                        let relative_path =
                            concat_paths!(path, format!(".{}.js", &dep[prefix_path.len()..]))
                                .display()
                                .to_string();

                        // When there is a first match, we break
                        if Path::new(&relative_path).is_file() {
                            resolved_path = Some(relative_path);
                            break;
                        }
                    }

                    if let Some(path) = resolved_path {
                        path
                    } else {
                        log::warn!("Couldn't find a file corresponding to {dep:#?}");
                        continue;
                    }
                }
            }
            // Else it's just a package
            else {
                dep
            };


            relatives_dependencies.push(dep_relative_path.clone());

            // If it's not a package, we need to look at the dependencies of this file
            if !is_dep_package && let Some(sub_deps) = script_dependencies(&dep_relative_path) {
                for sub_dep in sub_deps {
                    if relatives_dependencies.contains(&sub_dep) {
                        let maybe_index = relatives_dependencies.iter().position(|d| *d == sub_dep);
                        if let Some(index) = maybe_index {
                            relatives_dependencies.remove(index);
                        }
                    }

                    relatives_dependencies.push(sub_dep);
                }
            }
        }

        Some(relatives_dependencies)
    }
}
