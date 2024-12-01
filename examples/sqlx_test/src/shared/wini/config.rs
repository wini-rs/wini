use {
    super::{cache::CacheCategory, dependencies::normalize_relative_path, env::EnvType, ENV_TYPE},
    crate::concat_paths,
    serde::{Deserialize, Deserializer},
    std::{
        collections::HashMap,
        fmt::Display,
        io,
        str::FromStr,
        sync::{Arc, LazyLock},
    },
    strum::IntoEnumIterator,
};


/// The config parsed from `./wini.toml`
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub path: ConfigPath,
    pub cache: Caches,
}

impl Config {
    /// Load the configuration from the `./wini.toml` file at the root of the project
    pub fn from_file() -> Result<Self, TomlLoadingError> {
        let file_to_read_from = "./wini.toml";
        toml::from_str(
            std::fs::read_to_string(file_to_read_from)
                .map_err(|err| {
                    match err.kind() {
                        io::ErrorKind::NotFound => {
                            TomlLoadingError::ConfigFileDoesntExists(file_to_read_from.to_owned())
                        },
                        _ => TomlLoadingError::OtherIo(err),
                    }
                })?
                .as_ref(),
        )
        .map_err(|err| TomlLoadingError::InvalidToml(err, file_to_read_from.to_owned()))
    }
}


/// The paths of different important folders
#[derive(Debug, serde::Deserialize)]
pub struct ConfigPath {
    pub pages: String,
    pub layout: String,
    pub public: String,
    pub components: String,
    pub modules: String,
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
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        // Check if the value is an object
        if let serde_json::Value::Object(map) = value {
            let mut cache_categories_rules = HashMap::new();

            for (key, val) in map {
                if val.is_boolean() && key == "function" {
                    continue;
                }

                let cache_rule: String =
                    serde_json::from_value(val).map_err(serde::de::Error::custom)?;

                let cache_config = CacheCategory::from_str(&key)
                    .map_err(|e| serde::de::Error::custom(format!("Invalid key: `{key}`\n{e}")))?;

                cache_categories_rules.insert(cache_config, cache_rule);
            }

            Ok(ConfigCache(cache_categories_rules))
        } else {
            Err(serde::de::Error::custom("expected an object"))
        }
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
    pub fn get(&self, cache_for: CacheCategory) -> String {
        self.get_opt(cache_for).unwrap()
    }

    /// Get the current cache rule for a specific cache category
    pub fn get_opt(&self, cache_for: CacheCategory) -> Option<String> {
        self.get_opt_with_env_type(*ENV_TYPE, cache_for)
    }

    fn get_opt_with_env_type(&self, env_type: EnvType, cache_for: CacheCategory) -> Option<String> {
        self.environments
            .get(&env_type)
            .and_then(|env| env.as_ref())
            .and_then(|env| env.0.get(&cache_for))
            .cloned()
            .or_else(|| {
                self.default
                    .as_ref()
                    .and_then(|env| env.0.get(&cache_for))
                    .cloned()
            })
    }

    /// Verify that all the cache categories have a cache rule associated to them
    pub fn verify_all_attributes(&self) {
        for env in EnvType::iter() {
            for cache_for in CacheCategory::iter() {
                // Function category is only used by macros to know if you want to precompute
                // #[cache] functions.
                if cache_for != CacheCategory::Function &&
                    self.get_opt_with_env_type(env, cache_for).is_none()
                {
                    log::error!(
                        "\
                    The cache for {cache_for:#?} isn't defined in the environment {env:#?}.\n\
                    Look at your cache definitions in `./wini.toml`\
                        "
                    );
                    std::process::exit(1);
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum TomlLoadingError {
    ConfigFileDoesntExists(String),
    InvalidToml(toml::de::Error, String),
    OtherIo(io::Error),
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


pub static SERVER_CONFIG: LazyLock<Arc<Config>> = LazyLock::new(|| {
    Arc::new(Config::from_file().unwrap_or_else(|error| {
        log::error!("{error}");
        log::info!("Terminating program...");
        std::process::exit(1);
    }))
});
