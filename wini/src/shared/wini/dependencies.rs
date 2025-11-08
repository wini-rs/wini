use {
    super::{
        JS_FILES,
        err::ExitWithMessageIfErr,
        tsconfig::{TSCONFIG_PATHS, TsConfigPathsPrefix},
    },
    crate::concat_paths,
    regex::Regex,
    std::{
        collections::{HashMap, VecDeque},
        ops::Not,
        path::{Component, Path, PathBuf},
        sync::LazyLock,
    },
};

pub static REGEX_DEPENDENCY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(import|from)\s*["']([^'"]+)["'](;|\n)"#)
        .exit_with_msg_if_err("This should always be a valid regex.")
});

pub static SCRIPTS_DEPENDENCIES: LazyLock<HashMap<String, Option<Vec<String>>>> =
    LazyLock::new(|| {
        JS_FILES
            .keys()
            .map(|script| (script.to_owned(), script_dependencies(script)))
            .collect()
    });


/// Normalizes a relative file path by resolving `.` (current directory) and `..` (parent directory) components.
///
/// # Example
///
/// ```rust
/// use PROJECT_NAME_TO_RESOLVE::shared::wini::dependencies::normalize_relative_path;
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new("./folder/../file.txt");
/// let normalized = normalize_relative_path(path);
/// assert_eq!(normalized, PathBuf::from("file.txt"));
/// ```
///
/// _Equivalent of unstable [`std::path::Path::normalize_lexically`]_
pub fn normalize_relative_path(path: impl AsRef<Path>) -> PathBuf {
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
            Component::RootDir | Component::Normal(_) | Component::Prefix(_) => {
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
/// # Returns
/// An `Option<Vec<String>>`. The order of dependency matters, therefore a [`Vec`] is more suitable
/// than a [`HashSet`].
///
/// # Example:
/// `file1.js` // import "./file2";
/// `file2.js` // import "debug";
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
/// ```
///
/// # Panic
///
/// If there is an error finding a dependency
fn script_dependencies(path: &str) -> Option<Vec<String>> {
    let mut all_dependencies = Vec::new();
    let mut visited = HashMap::<String, u16>::new();
    let mut to_process = VecDeque::new();

    let normalized_path = normalize_relative_path(path).display().to_string();
    to_process.push_back(normalized_path.clone());

    while let Some(current_path) = to_process.pop_front() {
        if let Some(deps) = extract_dependencies(&current_path) {
            for ResolvedDependency {
                path: dep_path,
                is_external_package,
            } in deps
            {
                if !is_external_package {
                    to_process.push_back(dep_path.clone());
                }

                all_dependencies.retain(|dep| *dep != dep_path);
                all_dependencies.push(dep_path);
            }
        }

        if *visited
            .entry(current_path)
            .and_modify(|v| *v += 1)
            .or_default() ==
            1000
        {
            log::error!("Infinite recursion found in Typescript imports.");
            panic!("End of program");
        }
    }

    all_dependencies.retain(|dep| *dep != normalized_path);

    all_dependencies.reverse();

    all_dependencies
        .is_empty()
        .not()
        .then_some(all_dependencies)
}


fn extract_dependencies(path: &str) -> Option<Vec<ResolvedDependency>> {
    let path = find_existing_path(path);
    let contents = std::fs::read_to_string(&path).exit_with_msg_if_err("IO Error");

    let caps = REGEX_DEPENDENCY.captures_iter(&contents);
    let dependencies = caps
        .into_iter()
        .map(|m| m.extract::<3>())
        .map(|ex| ex.1[1].to_string())
        .filter_map(|dep| resolve_dependency_path(&dep, &path))
        .collect::<Vec<_>>();

    if dependencies.is_empty() {
        None
    } else {
        Some(dependencies)
    }
}

fn resolve_dependency_path(dep: &str, path: &Path) -> Option<ResolvedDependency> {
    // If an import starts with a ".", it's a path to a file. In this case, we want to
    // have it's path relative to the file it's referenced from.
    if dep.starts_with('.') {
        let dep = if dep.ends_with(".js") || dep.ends_with(".ts") {
            concat_paths!(path.parent().expect("Path should have a parent."), dep)
        } else {
            log::warn!("File {dep:?} doesn't have a defined extension. Trying `.ts`...");
            concat_paths!(
                path.parent().expect("Path should have a parent."),
                &format!("{dep}.ts")
            )
        };

        Some(ResolvedDependency {
            path: normalize_relative_path(dep).display().to_string(),
            is_external_package: false,
        })
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
        if let Some(first) = vec.first() {
            Some(ResolvedDependency {
                path: concat_paths!(first, &dep[prefix_path.len()..])
                    .display()
                    .to_string(),
                is_external_package: false,
            })
        } else {
            let mut resolved_path = None;

            for path in vec {
                let relative_path =
                    concat_paths!(path, format!(".{}.js", &dep[prefix_path.len()..]));

                // When there is a first match, we break
                if Path::new(&relative_path).is_file() {
                    resolved_path = Some(relative_path);
                    break;
                }
            }

            if let Some(path) = resolved_path {
                Some(ResolvedDependency {
                    path: path.display().to_string(),
                    is_external_package: false,
                })
            } else {
                log::warn!("Couldn't find a file corresponding to {dep:#?}");
                None
            }
        }
    }
    // Else it's just a package
    else {
        Some(ResolvedDependency {
            path: dep.to_owned(),
            is_external_package: true,
        })
    }
}

#[derive(Debug)]
struct ResolvedDependency {
    path: String,
    is_external_package: bool,
}

fn find_existing_path(path: &str) -> PathBuf {
    let base = path.strip_prefix('/').unwrap_or(path);

    for extension in [".ts", ".js"] {
        let candidate = PathBuf::from(base).with_extension(extension);
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from(base)
}

#[cfg(test)]
mod normalize_relative_path_tests {
    use {super::*, std::path::PathBuf};

    #[test]
    fn test_simple_relative_path() {
        let path = Path::new("./folder/file.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("folder/file.txt"));
    }

    #[test]
    fn test_parent_directory_removal() {
        let path = Path::new("./folder/../file.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("file.txt"));
    }

    #[test]
    fn test_multiple_parent_directories() {
        let path = Path::new("./folder1/../folder2/../file.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("file.txt"));
    }

    #[test]
    fn test_leading_parent_directory() {
        let path = Path::new("../file.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("../file.txt"));
    }

    #[test]
    fn test_root_directory() {
        let path = Path::new("/folder/../file.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("/file.txt"));
    }

    #[test]
    fn test_current_directory_only() {
        let path = Path::new(".");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::new());
    }

    #[test]
    fn test_only_parent_directory() {
        let path = Path::new("..");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from(".."));
    }

    #[test]
    fn test_multiple_current_directories() {
        let path = Path::new("././folder/./file.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("folder/file.txt"));
    }

    #[test]
    fn test_full_path() {
        let path = Path::new("/a/b/c/../../file.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("/a/file.txt"));
    }

    #[test]
    fn test_no_normalization_needed() {
        let path = Path::new("already/normalized/path.txt");
        let normalized = normalize_relative_path(path);
        assert_eq!(normalized, PathBuf::from("already/normalized/path.txt"));
    }
}

#[cfg(test)]
mod tests_script_dependencies {
    use super::*;

    #[test]
    fn example1() {
        let deps = script_dependencies("./src/shared/wini/tests/dependencies/example1/a.js");
        assert_eq!(
            deps,
            Some(vec![
                "c.js".into(),
                "src/shared/wini/tests/dependencies/example1/b.js".into(),
            ])
        );
    }

    #[test]
    fn example2() {
        let deps = script_dependencies("./src/shared/wini/tests/dependencies/example2/a.ts");
        assert_eq!(
            deps,
            Some(vec![
                "test".into(),
                "src/shared/wini/tests/dependencies/example2/d.js".into(),
                "src/shared/wini/tests/dependencies/example2/c.ts".into(),
                "src/shared/wini/tests/dependencies/example2/b.js".into(),
            ])
        );
    }

    #[test]
    fn example3() {
        let deps = script_dependencies("./src/shared/wini/tests/dependencies/example3/a.ts");
        assert_eq!(
            deps,
            Some(vec![
                "f".into(),
                "e".into(),
                "d".into(),
                "c".into(),
                "b".into(),
                "src/shared/wini/tests/dependencies/example3/f.ts".into(),
                "src/shared/wini/tests/dependencies/example3/e.ts".into(),
                "src/shared/wini/tests/dependencies/example3/d.ts".into(),
                "src/shared/wini/tests/dependencies/example3/c.ts".into(),
                "src/shared/wini/tests/dependencies/example3/b.ts".into(),
            ])
        );
    }

    #[test]
    fn example4() {
        let deps = script_dependencies("./src/shared/wini/tests/dependencies/example4/a.ts");
        assert_eq!(
            deps,
            Some(vec![
                "src/shared/wini/tests/dependencies/example4/b.ts".into(),
            ])
        );
    }

    #[test]
    fn example5() {
        let deps = script_dependencies("./src/shared/wini/tests/dependencies/example5/a.ts");
        assert_eq!(
            deps,
            Some(vec![
                "test".into(),
                "src/shared/wini/tests/dependencies/example5/c.ts".into(),
                "random".into(),
                "src/shared/wini/tests/dependencies/example5/b.ts".into(),
                "src/shared/wini/tests/dependencies/example5/d.ts".into(),
            ])
        );
    }
}
