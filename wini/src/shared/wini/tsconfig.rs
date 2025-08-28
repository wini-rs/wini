//! Handle topics linked to `tsconfig.json`

use {
    serde::Deserialize,
    std::{collections::HashMap, sync::LazyLock},
};

#[derive(Deserialize, Debug)]
struct CompilerOptions {
    paths: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    compiler_options: CompilerOptions,
}


/// Get all the abbreviations of a path from `./tsconfig.json`.
///
/// If tsconfig.json doesn't exist - for some reason - an empty HashMap will be returned.
pub static TSCONFIG_PATHS: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(|| {
    let tsconfig = match std::fs::read_to_string("./tsconfig.json") {
        Ok(s) => s,
        Err(why) => {
            log::warn!("{why:#?}");
            return HashMap::new();
        },
    };

    let config: TsConfig = serde_json::from_str(&tsconfig).expect("Failed to parse JSON");

    let paths = config.compiler_options.paths.unwrap_or_default();

    paths
        .into_iter()
        .map(|(k, v)| {
            let key = k.strip_suffix("/*").unwrap_or(&k).to_string();
            let values = v
                .into_iter()
                .map(|path| path.strip_suffix("/*").unwrap_or(&path).to_string())
                .collect();
            (key, values)
        })
        .collect()
});

pub trait TsConfigPathsPrefix {
    /// Get all the prefixes that should be resolved in a `tsconfig.json` file
    fn prefixes(&self) -> Vec<&str>;
}

impl TsConfigPathsPrefix for HashMap<String, Vec<String>> {
    fn prefixes(&self) -> Vec<&str> {
        let mut prefixes = self
            .keys()
            .map(std::string::String::as_str)
            .collect::<Vec<&str>>();
        // We sort them from the longest to shortest, because we need to first get the complete
        // path.
        // In case of:
        // ["~/utils/*", "~/*"]
        // utils needs to be resolved first
        prefixes.sort_by_key(|k| std::cmp::Reverse(k.len()));
        prefixes
    }
}
