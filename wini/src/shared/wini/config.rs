use {
    super::{cache::CacheCategory, dependencies::normalize_relative_path, env::EnvType, ENV_TYPE},
    crate::{
        concat_paths,
        shared::wini::err::ExitWithMessageIfErr,
        utils::wini::file::toml_from_path_as_static_str,
    },
    getset::Getters,
    serde::{Deserialize, Deserializer},
    std::{collections::HashMap, fmt::Display, io, str::FromStr, sync::LazyLock},
    strum::IntoEnumIterator,
};


pub static SERVER_CONFIG: LazyLock<Config> =
    LazyLock::new(|| Config::from_file().exit_with_msg_if_err("Invalid config"));


/// The config parsed from `./wini.toml`
#[derive(Debug, serde::Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Config {
    path: ConfigPath,
    cache: Caches,
}

impl Config {
    /// Load the configuration from the `./wini.toml` file at the root of the project
    pub fn from_file() -> Result<Self, TomlLoadingError> {
        toml_from_path_as_static_str("./wini.toml")
    }
}


/// The paths of different important folders
/// ConfigPath uses [`String`] instead of [`std::path::PathBuf`] becase we often use
/// [`ToString::to_string`].
#[derive(Debug, serde::Deserialize, Getters)]
#[getset(get = "pub")]
pub struct ConfigPath {
    pages: String,
    layouts: String,
    public: String,
    components: String,
    modules: String,
}

impl ConfigPath {
    pub fn public_from_src(&self) -> String {
        let path = concat_paths!("src", &self.public);
        normalize_relative_path(path).display().to_string()
    }
}


#[derive(Debug)]
struct ConfigCache(HashMap<CacheCategory, String>);

impl<'de> Deserialize<'de> for ConfigCache {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let table: toml::Table = Deserialize::deserialize(deserializer)?;

        let mut cache_categories_rules = HashMap::new();

        for (key, val) in table {
            if val.is_bool() && key == "function" {
                continue;
            }

            let cache_rule: String = match val {
                toml::Value::String(string) => string,
                _ => {
                    return Err(serde::de::Error::custom(
                        "Values of cache rules should be strings",
                    ));
                },
            };

            let cache_config = CacheCategory::from_str(&key)
                .map_err(|e| serde::de::Error::custom(format!("Invalid key: `{key}`\n{e}")))?;

            cache_categories_rules.insert(cache_config, cache_rule);
        }

        Ok(ConfigCache(cache_categories_rules))
    }
}


/// The cache options for different kind of environments
#[derive(Debug, serde::Deserialize)]
pub struct Caches {
    default: Option<ConfigCache>,
    #[serde(flatten)]
    environments: HashMap<EnvType, Option<ConfigCache>>,
}

impl Caches {
    /// Get the current cache rule for a specific cache category
    pub fn get_or_panic(&self, cache_for: CacheCategory) -> &str {
        self.get(cache_for).unwrap_or_else(|| {
            panic!(
                "Cache rule for {cache_for:?} not found in environment {env:?}. \
                Check your ./wini.toml configuration.",
                env = *ENV_TYPE
            )
        })
    }

    /// Get the current cache rule for a specific cache category
    pub fn get(&self, cache_for: CacheCategory) -> Option<&str> {
        self.get_opt_with_env_type(*ENV_TYPE, cache_for)
    }

    fn get_opt_with_env_type(&self, env_type: EnvType, cache_for: CacheCategory) -> Option<&str> {
        let env_from = match self.environments.get(&env_type) {
            Some(env) => env,
            None => &self.default,
        };

        env_from
            .as_ref()
            .and_then(|env| env.0.get(&cache_for))
            .map(AsRef::as_ref)
    }

    /// Verify that all the cache categories have a cache rule associated to them
    pub fn verify_all_attributes(&self) {
        for env in EnvType::iter() {
            for cache_for in CacheCategory::iter() {
                // Function category is only used by macros to know if you want to precompute
                // #[cached] functions.
                if cache_for != CacheCategory::Function &&
                    self.get_opt_with_env_type(env, cache_for).is_none()
                {
                    log::error!(
                        "\
                    The cache for {cache_for:#?} isn't defined in the environment {env:#?}.\n\
                    Look at your cache definitions in `./wini.toml`\
                        "
                    );
                    panic!("End of program")
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum TomlLoadingError {
    ConfigFileDoesntExists(&'static str),
    InvalidToml(toml::de::Error, &'static str),
    OtherIo(io::Error),
}

impl std::error::Error for TomlLoadingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidToml(err, _) => Some(err),
            Self::OtherIo(err) => Some(err),
            Self::ConfigFileDoesntExists(_) => None,
        }
    }
}

impl Display for TomlLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlLoadingError::InvalidToml(err, filename) => {
                write!(
                    f,
                    "`{filename}` seems to have an invalid configuration!\n{err}",
                )
            },
            TomlLoadingError::ConfigFileDoesntExists(filename) => {
                write!(f, "No file `{filename}`.")
            },
            TomlLoadingError::OtherIo(err) => {
                write!(f, "{err}")
            },
        }
    }
}
