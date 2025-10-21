#![feature(proc_macro_span)]

use {
    dotenvy::dotenv,
    proc_macro::TokenStream,
    std::{collections::HashMap, sync::LazyLock},
};

mod macros;
pub(crate) mod utils;

/// Creates a wini component
#[proc_macro_attribute]
pub fn component(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::wini::component::component(args, item)
}

/// Wraps pages or other layouts with common HTML structure.
///
/// Layouts are applied as Axum middleware and can receive child content in various forms.
/// They automatically link JS/CSS files and support SEO meta tags.
///
/// # Parameters
///
/// - `title` - Page title (sets `<title>` and `og:title`)
/// - `description` - Meta description (sets `description` and `og:description`)
/// - `keywords` - Array of keywords for SEO
/// - `author` - Content author
/// - `site_name` - Site name for Open Graph
/// - `lang` - Language code (e.g., "en", "fr")
/// - `img` - Open Graph image URL
/// - `robots` - Robot indexing instructions
/// - `js_pkgs` - Array of JavaScript package names to include
/// - `other_meta` - Array of custom meta tag key-value pairs
///
/// # Layout Input Types
///
/// Layouts can accept different parameter types depending on your needs:
///
/// - `&str` - Receives rendered child HTML as string
/// - `StatusCode` - Receives HTTP status code (useful for error layouts)
/// - `&Parts` - Receives HTTP response parts (read-only)
/// - `&mut Parts` - Receives mutable HTTP response parts
/// - `&Parts, &Body` - Receives both parts and body
///
/// # Examples
///
/// ## Basic usage with string content
///
/// ```rust,ignore
/// use {maud::{html, Markup, PreEscaped}, wini_macros::layout};
///
/// #[layout]
/// pub async fn main_layout(child: Markup) -> Markup {
///     html! {
///         header { "Site Header" }
///         main { (PreEscaped(child)) }
///         footer { "Site Footer" }
///     }
/// }
/// ```
///
/// ## With HTTP parts (accessing request info)
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::layout, axum::http::response::Parts};
///
/// #[layout]
/// pub async fn debug_layout(parts: &mut Parts) -> ServerResult<Markup> {
///     let status = parts.status;
///     Ok(html! {
///         div class="debug-wrapper" {
///             p { "Status: " (status.as_u16()) }
///             p { "Version: " (format!("{:?}", parts.version)) }
///         }
///     })
/// }
/// ```
///
/// ## With status code (error pages)
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::layout, hyper::StatusCode};
///
/// #[layout]
/// pub async fn error_layout(status: StatusCode) -> Markup {
///     html! {
///         div class="error-page" {
///             h1 { "Error " (status.as_u16()) }
///             @match status {
///                 StatusCode::NOT_FOUND => {
///                     p { "The page you're looking for doesn't exist." }
///                 }
///                 StatusCode::INTERNAL_SERVER_ERROR => {
///                     p { "Something went wrong on our end." }
///                 }
///                 _ => {
///                     p { "An error occurred." }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## With SEO meta tags
///
/// ```rust,ignore
/// use {maud::{html, Markup, PreEscaped}, wini_macros::layout};
///
/// #[layout(
///     title = "My Website",
///     description = "A website built with Wini",
///     lang = "en",
///     site_name = "Wini Framework"
/// )]
/// pub async fn seo_layout(child: Markup) -> Markup {
///     html! {
///         main {
///             (PreEscaped(child))
///         }
///     }
/// }
/// ```
///
/// ## With JavaScript packages
///
/// ```rust,ignore
/// use {maud::{html, Markup, PreEscaped}, wini_macros::layout};
///
/// #[layout(js_pkgs = ["htmx"])]
/// pub async fn htmx_layout(child: Markup) -> Markup {
///     html! {
///         div hx-boost="true" {
///             nav {
///                 a href="/" { "Home" }
///                 a href="/about" { "About" }
///             }
///             main { (PreEscaped(child)) }
///         }
///     }
/// }
/// ```
///
/// ## Nested layouts
///
/// ```rust,ignore
/// use {maud::{html, Markup, PreEscaped}, wini_macros::layout};
///
/// #[layout]
/// pub async fn base_layout(child: Markup) -> Markup {
///     html! {
///         main {
///             h1 { "Welcome back" }
///             (PreEscaped(child))
///         }
///     }
/// }
///
/// #[layout]
/// pub async fn auth_layout(child: Markup) -> Markup {
///     html! {
///         div class="auth-container" {
///             div class="auth-sidebar" {
///                 nav { "Auth Navigation" }
///             }
///             div class="auth-content" {
///                 (PreEscaped(child))
///             }
///         }
///     }
/// }
/// ```
///
/// ## With response parts and body
///
/// ```rust,ignore
/// use {
///     maud::{html, Markup},
///     wini_macros::layout,
///     axum::{body::Body, http::response::Parts}
/// };
///
/// #[layout]
/// pub async fn advanced_layout(parts: &Parts, body: &Body) -> ServerResult<Markup> {
///     let content_type = parts
///         .headers
///         .get("content-type")
///         .and_then(|v| v.to_str().ok())
///         .unwrap_or("unknown");
///     
///     Ok(html! {
///         div class="advanced-wrapper" {
///             p { "Content-Type: " (content_type) }
///         }
///     })
/// }
/// ```
#[proc_macro_attribute]
pub fn layout(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::wini::layout::layout(args, item)
}

/// Creates a wini page
///
/// # Usage
/// ```
/// #[page(
///     title = "Hello world!",
///     keywords = ["hello", "world"],
///     other_meta = [
///         "custom_meta1" = "hello",
///         "custom_meta2" = "world",
///     ]
/// )]
/// async fn render() -> Markup {
///     html! {
///         main {
///             h1 {
///                 "Hello world"
///             }
///             p {
///                 "Some really, really nice hello world!"
///             }
///         }
///     }
/// }
/// ```
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
