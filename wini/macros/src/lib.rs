#![feature(proc_macro_span)]

use {
    dotenvy::dotenv,
    proc_macro::TokenStream,
    std::{collections::HashMap, sync::LazyLock},
};

mod macros;
pub(crate) mod utils;

/// Creates a reusable HTML component that can be composed within pages or other components.
///
/// Unlike `#[page]`, components return `Markup` directly and are not converted to HTTP responses.
/// They automatically link JS/CSS files and can accept parameters.
///
/// # Parameters
///
/// - `js_pkgs` - Array of JavaScript package names to include
///
/// # Return Types
///
/// The function can return either `Markup` or `ServerResult<Markup>` for error handling.
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::component};
///
/// #[component]
/// pub async fn button() -> Markup {
///     html! {
///         button class="btn" {
///             "Click me!"
///         }
///     }
/// }
/// ```
///
/// ## With parameters
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::component};
///
/// #[component]
/// pub async fn card(title: String, content: String) -> Markup {
///     html! {
///         div class="card" {
///             h2 { (title) }
///             p { (content) }
///         }
///     }
/// }
/// ```
///
/// ## With error handling
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::component};
///
/// #[component]
/// pub async fn user_card(user_id: i32) -> ServerResult<Markup> {
///     let user = fetch_user(user_id).await?;
///     Ok(html! {
///         div class="user-card" {
///             h3 { (user.name) }
///             p { (user.email) }
///         }
///     })
/// }
/// ```
///
/// ## Using components in pages
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::{page, component}};
///
/// #[component]
/// pub async fn nav() -> Markup {
///     html! {
///         nav {
///             a href="/" { "Home" }
///             a href="/about" { "About" }
///         }
///     }
/// }
///
/// #[page]
/// pub async fn dashboard() -> ServerResult<Markup> {
///     Ok(html! {
///         (nav().await?)
///         main {
///             h1 { "Dashboard" }
///         }
///     })
/// }
/// ```
///
/// ## With JavaScript packages
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::component};
///
/// #[component(js_pkgs = ["alpinejs"])]
/// pub async fn counter() -> Markup {
///     html! {
///         div x-data="{ count: 0 }" {
///             button x-on:click="count++" { "Increment" }
///             span x-text="count" {}
///             button x-on:click="count--" { "Decrement" }
///         }
///     }
/// }
/// ```
///
/// ## Composing multiple components
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::component};
///
/// #[component]
/// pub async fn icon(name: String) -> Markup {
///     html! {
///         i class={"icon icon-" (name)} {}
///     }
/// }
///
/// #[component]
/// pub async fn button_with_icon(icon_name: String, label: String) -> ServerResult<Markup> {
///     Ok(html! {
///         button class="btn" {
///             (icon(icon_name).await?)
///             span { (label) }
///         }
///     })
/// }
/// ```
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
/// pub async fn main_layout(child: &str) -> Markup {
///     html! {
///         html {
///             head {
///                 title { "My Site" }
///                 meta charset="utf-8";
///             }
///             body {
///                 header { "Site Header" }
///                 main { (PreEscaped(child)) }
///                 footer { "Site Footer" }
///             }
///         }
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
///         html {
///             body class="error-page" {
///                 h1 { "Error " (status.as_u16()) }
///                 @match status {
///                     StatusCode::NOT_FOUND => {
///                         p { "The page you're looking for doesn't exist." }
///                     }
///                     StatusCode::INTERNAL_SERVER_ERROR => {
///                         p { "Something went wrong on our end." }
///                     }
///                     _ => {
///                         p { "An error occurred." }
///                     }
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
/// pub async fn seo_layout(child: &str) -> Markup {
///     html! {
///         html lang="en" {
///             head {
///                 meta charset="utf-8";
///                 meta name="viewport" content="width=device-width, initial-scale=1";
///             }
///             body {
///                 (PreEscaped(child))
///             }
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
/// pub async fn htmx_layout(child: &str) -> Markup {
///     html! {
///         html {
///             body hx-boost="true" {
///                 nav {
///                     a href="/" { "Home" }
///                     a href="/about" { "About" }
///                 }
///                 main { (PreEscaped(child)) }
///             }
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
/// pub async fn base_layout(child: &str) -> Markup {
///     html! {
///         html {
///             head {
///                 link rel="stylesheet" href="/styles/base.css";
///             }
///             body {
///                 (PreEscaped(child))
///             }
///         }
///     }
/// }
///
/// #[layout]
/// pub async fn auth_layout(child: &str) -> Markup {
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

/// Transforms an async function returning `Markup` into a complete HTTP response handler.
///
/// The `page` macro automatically handles:
/// - Conversion to `axum::response::Response`
/// - Linking JS/CSS files from the current directory
/// - Injecting SEO meta tags
/// - JavaScript package management
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
/// - `robots` - Robot indexing instructions (e.g., "index, follow")
/// - `js_pkgs` - Array of JavaScript package names to include
/// - `other_meta` - Array of custom meta tag key-value pairs
///
/// # Return Types
///
/// The function can return either `Markup` or `ServerResult<Markup>` for error handling.
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::page};
///
/// #[page]
/// pub async fn index() -> Markup {
///     html! {
///         h1 { "Hello, World!" }
///     }
/// }
/// ```
///
/// ## With route parameters
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::page};
///
/// #[page]
/// pub async fn user_profile(user_id: String) -> Markup {
///     html! {
///         h1 { "User: " (user_id) }
///     }
/// }
/// ```
///
/// ## With error handling
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::page};
///
/// #[page]
/// pub async fn fallible_page() -> ServerResult<Markup> {
///     let data = fetch_data().await?;
///     Ok(html! {
///         p { (data) }
///     })
/// }
/// ```
///
/// ## With SEO meta tags
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::page};
///
/// #[page(
///     title = "My Page Title",
///     description = "Page description for SEO",
///     keywords = ["rust", "web", "framework"],
///     author = "Your Name",
///     lang = "en",
///     img = "/og-image.png",
///     robots = "index, follow",
///     site_name = "My Site"
/// )]
/// pub async fn seo_page() -> Markup {
///     html! {
///         h1 { "SEO-optimized page" }
///     }
/// }
/// ```
///
/// ## With JavaScript packages
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::page};
///
/// #[page(js_pkgs = ["alpinejs", "htmx"])]
/// pub async fn interactive_page() -> Markup {
///     html! {
///         div x-data="{ open: false }" {
///             button x-on:click="open = !open" { "Toggle" }
///         }
///     }
/// }
/// ```
///
/// ## With custom meta tags
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::page};
///
/// #[page(
///     title = "Custom Meta Tags",
///     other_meta = [
///         "theme-color" = "#3B82F6",
///         "custom:property" = "value"
///     ]
/// )]
/// pub async fn custom_meta() -> Markup {
///     html! {
///         h1 { "Page with custom meta tags" }
///     }
/// }
/// ```
///
/// ## Complete example with all parameters
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::page};
///
/// #[page(
///     title = "Complete Example",
///     description = "A page with all parameters",
///     keywords = ["example", "documentation"],
///     author = "Jane Doe",
///     site_name = "Wini Framework",
///     lang = "en",
///     img = "/images/og-image.png",
///     robots = "index, follow",
///     js_pkgs = ["alpinejs", "htmx"],
///     other_meta = [
///         "theme-color" = "#3B82F6"
///     ]
/// )]
/// pub async fn complete_example() -> Markup {
///     html! {
///         h1 { "Complete example" }
///         p { "This page has all available parameters configured." }
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
