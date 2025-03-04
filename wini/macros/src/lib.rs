#![feature(proc_macro_span)]

use {
    dotenvy::dotenv,
    proc_macro::TokenStream,
    std::{collections::HashMap, sync::LazyLock},
};

mod macros;
pub(crate) mod utils;

#[proc_macro_attribute]
pub fn component(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::wini::component::component(args, item)
}

#[proc_macro_attribute]
pub fn wrapper(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::wini::layout::layout(args, item)
}

#[proc_macro_attribute]
pub fn layout(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::wini::layout::layout(args, item)
}

#[proc_macro_attribute]
pub fn page(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::wini::page::page(args, item)
}

#[proc_macro_attribute]
pub fn init_cache(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::wini::cache::init_cache(args, item)
}

/// Doesn't panic if there is an error. This will be the job of the server initialization to handle
/// that.
pub(crate) static SHOULD_CACHE_FN: LazyLock<bool> = LazyLock::new(|| {
    let toml = std::fs::read_to_string("./wini.toml").expect("Couldn't find `wini.toml`.");
    let toml: MinimalRepresentationOfWiniToml = match toml::from_str(&toml) {
        Ok(toml) => toml,
        Err(_) => return false,
    };

    dotenv().expect("Couldn't load environment");
    let env_type = match std::env::var("ENV_TYPE") {
        Ok(env_type) => env_type.to_lowercase(),
        Err(_) => return false,
    };


    toml.cache
        .environments
        .get(&env_type)
        .and_then(|maybe_config| maybe_config.as_ref().map(|c| c.function))
        .unwrap_or_else(|| toml.cache.default.is_some_and(|env| env.function))
});


#[derive(Debug, serde::Deserialize)]
struct ConfigCache {
    function: bool,
}

/// The cache options for different kind of environments
#[derive(Debug, serde::Deserialize)]
struct Caches {
    default: Option<ConfigCache>,
    #[serde(flatten)]
    environments: HashMap<String, Option<ConfigCache>>,
}

/// The config parsed from `./wini.toml`
#[derive(Debug, serde::Deserialize)]
struct MinimalRepresentationOfWiniToml {
    pub cache: Caches,
}
