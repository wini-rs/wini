#![feature(proc_macro_span)]

use {
    dotenvy::dotenv,
    proc_macro::TokenStream,
    std::{collections::HashMap, sync::LazyLock},
};

mod macros;
pub(crate) mod utils;

/// Creates a reusable HTML component that can be composed within pages, layouts or other components.
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
/// # Usage of a component
/// When you use a component, you should always include it with the following syntax of braces:
///
/// ```rust,ignore
/// // DO ✅
/// html! {
///     [my_component]
/// }
///
/// // DON'T ❌
/// html! {
///     (my_component().await)
/// }
/// ```
///
/// Even though the second one "works", it will not propagate the linked files. So, if `my_component`
/// depends on some style sheets, doing `(my_component().await)` will not include them!
///
/// ## Parameters
/// When the component takes parameters, you can just write it like that:
/// ```rust,ignore
/// #[component]
/// fn my_component(arg1: T1, arg2: T2) -> Markup { ... }
///
/// #[page]
/// fn my_page() {
///     let arg1 = ...;
///     let arg2 = ...;
///     html! {
///         [my_component(arg1, arg2)]
///     }
/// }
/// ```
///
/// ## Handling of `ServerResult<Markup>`
/// In some cases, your component might return a `ServerError`. In this case, you will need to use
/// a special syntax to specify how the error should be handled in the caller of the component.
///
/// There are 2 ways:
/// ```rust,ignore
/// #[component]
/// fn my_component() -> ServerResult<Markup> { ... }
///
/// #[page]
/// fn example_1() -> ServerResult<Markup> {
///     html! {
///         [my_component?]
///     }
/// }
///
/// #[page]
/// fn example_2() -> Markup {
///     html! {
///         [my_component!]
///     }
/// }
/// ```
/// 1. In `example_1` we use the `?` operator at the end. This has the same behaviour as in normal
///    Rust: it behaves as `return Err(...)`.
/// 2. In `example_2` we use the `!` operator at the end. This operator behaves exactly as
///    `.unwrap_or_default()`
///
/// If you want some more advanced handling of error you can do something like:
/// ```rust,ignore
/// #[component]
/// fn my_component() -> ServerResult<Markup> { ... }
///
/// fn process_error(component_result: ServerResult<Markup>) -> Markup {
///     component_result.unwrap_or_else(|_| html!("An error occurred!"))
/// }
///
/// #[page]
/// fn example_1() -> ServerResult<Markup> {
///     html! {
///         [process_error(my_component().await)]
///     }
/// }
/// ```
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
///         [nav]
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
///             [icon(icon_name)]
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
/// Layouts are applied as Axum middleware and can receive child content in various and
/// request/respone data.
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
/// # Layout argument types
///
/// Layouts can accept different argument types depending on your needs. Arguments must have a
/// type that either implements:
/// - [`axum::extract::FromRequestParts`],
/// - [`crate::shared::wini::response::FromResponseBody`]
/// - [`crate::shared::wini::response::FromResponseParts`]
///
/// There are just a few rules:
/// 1. Arguments that come from `FromResponseBody` MUST be the last argument.
/// 2. Only one argument can come from `FromResponseBody`.
/// 3. In case of conflicts (ex: an argument has a type of `http::header::HeaderMap`, which
///    implements both `FromRequestParts` and `FromResponseParts`), you can specify from which
///    implementation it should come from with the following macro attributes:
///    - `#[from_request_parts]`
///    - `#[from_response_parts]`
///    - `#[from_response_body]`
///
/// # Examples
///
/// ## Basic usage with string content
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::layout};
///
/// #[layout]
/// pub async fn main_layout(child: Markup) -> Markup {
///     html! {
///         header { "Site Header" }
///         main { (child) }
///         footer { "Site Footer" }
///     }
/// }
/// ```
///
/// ## With conflict between implementations
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::layout, axum::http::header::HeaderMap};
///
/// #[layout]
/// pub async fn using_both_header_maps(
///     #[from_request_parts] headers_req: HeaderMap,
///     #[from_response_parts] headers_resp: HeaderMap,
///     child: Markup,
/// ) -> Markup {
///     html! {
///         b { "Headers from request:" }
///         p { (format!("{headers_req:#?}")) }
///
///         b { "Headers from response:" }
///         p { (format!("{headers_resp:#?}")) }
///
///         (child)
///     }
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
/// use {maud::{html, Markup}, wini_macros::layout};
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
///             (child)
///         }
///     }
/// }
/// ```
///
/// ## With JavaScript packages
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::layout};
///
/// #[layout(js_pkgs = ["htmx"])]
/// pub async fn htmx_layout(child: Markup) -> Markup {
///     html! {
///         div hx-boost="true" {
///             nav {
///                 a href="/" { "Home" }
///                 a href="/about" { "About" }
///             }
///             main { (child) }
///         }
///     }
/// }
/// ```
///
/// ## Nested layouts
///
/// ```rust,ignore
/// use {maud::{html, Markup}, wini_macros::layout};
///
/// #[layout]
/// pub async fn base_layout(child: Markup) -> Markup {
///     html! {
///         main {
///             h1 { "Welcome back" }
///             (child)
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
///                 (child)
///             }
///         }
///     }
/// }
/// ```
///
/// ## With error backtrace handling
///
/// ```rust,ignore
/// use {
///     maud::{html, Markup},
///     wini_macros::layout,
///     wini::shared::wini::err::Backtrace,
/// };
///
/// #[layout]
/// pub async fn error_handler(
///     backtrace: Option<Backtrace>,
///     child: Markup
/// ) -> ServerResult<Markup> {
///     Ok(html! {
///         div class="error-layout" {
///             @if let Some(bt) = backtrace {
///                 div class="error-info" {
///                     p { "An error occurred: " (format!("{bt:#?}")) }
///                 }
///             }
///             main { (child) }
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
/// - Propagate errors if there are some
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
