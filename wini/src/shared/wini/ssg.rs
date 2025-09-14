//! Static Site Generation (SSG) utilities for pre-rendering routes to static files.
//!
//! This module provides tools to define routes with optional parameters and render them
//! as static HTML files with associated assets, enabling deployment to static hosting services.

use {
    crate::shared::wini::{config::SERVER_CONFIG, PORT},
    axum::{routing::MethodRouter, Router},
    reqwest::Client,
    select::{document::Document, predicate::Name},
    std::{
        borrow::Cow,
        collections::{HashMap, HashSet},
        path::{Path, PathBuf},
        sync::{Arc, LazyLock, Mutex},
    },
};

/// A router builder for Static Site Generation that tracks routes and their parameter variants.
///
/// `SsgRouter` allows you to register routes with optional parameter sets, then converts
/// them into an Axum router while tracking all concrete route paths for static rendering.
///
/// # Examples
///
/// ```rust,ignore
/// use {
///     axum::routing::get,
///     std::borrow::Cow,
/// };
///
/// let ssg_router = SsgRouter::new()
///     .route("/", get(home_handler))
///     .route_with_params(
///         "/blog/{slug}",
///         get(blog_post_handler),
///         vec![
///             vec![Cow::Borrowed("hello-world")],
///             vec![Cow::Borrowed("rust-tips")],
///         ]
///     );
///
/// // After that you can use it like
/// let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
///
/// axum::serve(listener, ssg_router.into_axum_router()).await.unwrap();
/// ```
///
/// All the routes that need to be SSG will be in `ROUTES_TO_AXUM`, in this case:
/// ```json
/// [
///     "/",
///     "/blog/hello-world",
///     "/blog/rust-tips",
/// ]
/// ```
#[derive(Debug, Default)]
pub(crate) struct SsgRouter<'l> {
    routes: HashMap<&'l str, (MethodRouter<()>, Option<Vec<Vec<Cow<'l, str>>>>)>,
}

impl<'l> SsgRouter<'l> {
    /// Creates a new SSG router without any route registered.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a static route without parameters.
    ///
    /// Use this for routes that don't contain dynamic segments (e.g., `/user/{id}`,
    /// `/blog/page/{title}`).
    ///
    /// # Arguments
    ///
    /// * `path` - The route path (must not contain `{param}` or `*wildcard` segments)
    /// * `m` - The Axum method router handling this path
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use axum::routing::get;
    ///
    /// let router = SsgRouter::new()
    ///     .route("/", get(home_handler))
    ///     .route("/about", get(about_handler));
    /// ```
    #[allow(unused, reason = "Not necessarily used")]
    pub fn route(mut self, path: &'l str, m: MethodRouter<()>) -> Self {
        self.routes.insert(path, (m, None));
        self
    }

    /// Registers a parameterized route with all possible parameter combinations.
    ///
    /// Use this for routes containing dynamic segments that you want to pre-render
    /// with specific parameter values.
    ///
    /// # Arguments
    ///
    /// * `path` - The route path containing `{param}` or `*wildcard` segments
    /// * `m` - The Axum method router handling this path
    /// * `params` - All parameter combinations to generate static files for.
    ///   Each inner `Vec` must contain exactly as many values as there are
    ///   dynamic segments in the path.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use {
    ///     axum::routing::get,
    ///     std::borrow::Cow,
    /// };
    ///
    /// // Route with one parameter
    /// let router = SsgRouter::new()
    ///     .route_with_params(
    ///         "/blog/{slug}",
    ///         get(blog_handler),
    ///         vec![
    ///             vec![Cow::Borrowed("post-1")],
    ///             vec![Cow::Borrowed("post-2")],
    ///         ]
    ///     );
    ///
    /// // Route with multiple parameters
    /// let router = SsgRouter::new()
    ///     .route_with_params(
    ///         "/blog/{year}/{slug}",
    ///         get(blog_handler),
    ///         vec![
    ///             vec![Cow::Borrowed("2024"), Cow::Borrowed("hello")],
    ///             vec![Cow::Borrowed("2024"), Cow::Borrowed("world")],
    ///         ]
    ///     );
    /// ```
    #[allow(unused, reason = "Not necessarily used")]
    pub fn route_with_params(
        mut self,
        path: &'l str,
        m: MethodRouter<()>,
        params: Vec<Vec<Cow<'l, str>>>,
    ) -> Self {
        self.routes.insert(path, (m, Some(params)));
        self
    }

    /// Converts the SSG router into an Axum router and registers all concrete routes for rendering.
    ///
    /// This method validates parameter counts against path segments and stores all
    /// concrete route paths in a global registry for later static generation.
    ///
    /// # Panics
    ///
    /// * If a parameterized route is registered without parameters
    /// * If any parameter set has the wrong number of values for its route
    /// * If a static route is registered with parameters
    pub fn into_axum_router(self) -> Router {
        let mut router = Router::new();

        for (path, (method_router, vec_params)) in self.routes {
            router = router.route(path, method_router);
            let path_segments = PathSegments::from_str(path);

            let nb_of_param_or_wildcard = path_segments.nb_of_param_or_wildcard();
            match vec_params {
                Some(vec_params) => {
                    if let Some(params) = vec_params
                        .iter()
                        .find(|params| params.len() != nb_of_param_or_wildcard)
                    {
                        panic!("For route `{path}`, expected {nb_of_param_or_wildcard} of parameters, got: {params_len} ({params:?})", params_len=params.len());
                    }

                    for params in vec_params {
                        ROUTES_TO_AXUM
                            .lock()
                            .unwrap()
                            .insert(path_segments.to_string_route(&params));
                    }
                },
                None => {
                    assert!(nb_of_param_or_wildcard == 0, "For route `{path}`, expected {nb_of_param_or_wildcard} of parameters, got: None", );
                    ROUTES_TO_AXUM.lock().unwrap().insert(path.to_owned());
                },
            }
        }

        router
    }
}

static ROUTES_TO_AXUM: LazyLock<Arc<Mutex<HashSet<String>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashSet::new())));

/// Renders all registered routes to static HTML files with their assets.
///
/// This function creates a `dist/` directory and generates static files for all routes
/// registered through `SsgRouter`. It:
///
/// 1. Fetches HTML content for each route from the running server
/// 2. Parses HTML to find local assets (CSS, JS)
/// 3. Downloads and saves assets preserving their directory structure
/// 4. Writes each route's HTML to `dist/{route}/index.html`
/// 5. Copies the entire public directory to `dist/`
///
/// # File Structure
///
/// Generated files follow this structure:
/// ```tree
/// dist/
/// ├── index.html              # Root route (/)
/// ├── about/
/// │   └── index.html          # /about route
/// ├── blog/
/// │   ├── post-1/
/// │   │   └── index.html      # /blog/post-1 route
/// │   └── post-2/
/// │       └── index.html      # /blog/post-2 route
/// ├── assets/
/// │   ├── style.css           # Downloaded assets
/// │   └── script.js
/// └── ...                     # Copied from public/
/// ```
///
/// # Errors
///
/// This function will panic if:
/// * The server is not reachable
/// * File system operations fail (permissions, disk space, etc.)
/// * HTML parsing fails
pub async fn render_routes_to_files() {
    std::fs::create_dir_all("dist/").unwrap();
    let mut static_assets = Vec::new();

    let routes = ROUTES_TO_AXUM.lock().unwrap().clone();

    let reqwest_client = Client::new();

    for route in &routes {
        let resp_text = reqwest_client
            .get(format!("http://localhost:{}{route}", *PORT))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let document = Document::from(resp_text.as_str());

        for link in document.find(Name("link")) {
            if let Some(href) = link.attr("href") && !href.starts_with("https://") {
            static_assets.push(href.to_owned());
        }
        }

        for script in document.find(Name("script")) {
            if let Some(src) = script.attr("src") && !src.starts_with("https://") {
            static_assets.push(src.to_owned());
        }
        }

        let mut path = PathBuf::from("dist");
        path.extend(route.split('/'));

        for comp in path.components() {
            assert!(
                matches!(comp, std::path::Component::Normal(_)),
                "Unsafe route component in `{route}`: {comp:?}"
            );
        }

        std::fs::create_dir_all(&path).unwrap();
        path.push("index.html");
        std::fs::write(path, resp_text).expect("Couldn't write the file");
    }

    for static_asset in static_assets {
        let resp_text = reqwest::get(format!("http://localhost:{}{static_asset}", *PORT))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut path = PathBuf::new();
        path.push("dist");
        path.extend(static_asset.split('/'));

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, resp_text).expect("Couldn't write the file");
    }

    copy_dir_all(SERVER_CONFIG.path.public_from_src(), "dist").unwrap();
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

enum PathSegment<'l> {
    String(&'l str),
    Param,
    Wildcard,
}

struct PathSegments<'l>(Vec<PathSegment<'l>>);

impl<'l> PathSegments<'l> {
    fn from_str(s: &'l str) -> Self {
        Self(
            s.split('/')
                .skip(1)
                .map(|segment| {
                    match segment.as_bytes().first() {
                        Some(b'*') => PathSegment::Wildcard,
                        Some(b':' | b'{') => {
                            if segment.as_bytes().get(1).copied() == Some(b'*') {
                                PathSegment::Wildcard
                            } else {
                                PathSegment::Param
                            }
                        },
                        _ => PathSegment::String(segment),
                    }
                })
                .collect(),
        )
    }

    fn nb_of_param_or_wildcard(&self) -> usize {
        self.0
            .iter()
            .filter(|seg| matches!(seg, PathSegment::Param | PathSegment::Wildcard))
            .count()
    }

    fn to_string_route(&'l self, params: &[Cow<'l, str>]) -> String {
        let mut current_idx_params = 0;
        self.0.iter().fold(String::new(), |mut acc, it| {
            acc.push('/');
            acc.push_str(match it {
                PathSegment::String(s) => s,
                PathSegment::Wildcard | PathSegment::Param => {
                    current_idx_params += 1;
                    params
                        .get(current_idx_params - 1)
                        .expect("current_idx_params starts at 0, params length has been verified")
                },
            });
            acc
        })
    }
}
