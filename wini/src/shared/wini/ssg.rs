use {
    crate::shared::wini::PORT,
    axum::{handler::Handler, routing::MethodRouter, Router},
    select::{document::Document, predicate::Name},
    std::{
        borrow::Cow,
        collections::{HashMap, HashSet},
        path::{Path, PathBuf},
        sync::{Arc, LazyLock, Mutex},
    },
};

/// A basic router for SSG
#[derive(Debug, Default)]
pub(crate) struct SsgRouter<'l> {
    routes: HashMap<&'l str, (MethodRouter<()>, Option<Vec<Vec<Cow<'l, str>>>>)>,
}

impl<'l> SsgRouter<'l> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn route(mut self, path: &'l str, m: MethodRouter<()>) -> Self {
        self.routes.insert(path, (m, None));
        self
    }

    pub fn route_with_params(
        mut self,
        path: &'l str,
        m: MethodRouter<()>,
        params: Vec<Vec<Cow<'l, str>>>,
    ) -> Self {
        self.routes.insert(path, (m, Some(params)));
        self
    }

    pub fn into_axum_router(self) -> Router {
        let mut router = Router::new();

        for (path, (method_router, vec_params)) in self.routes {
            router = router.route(path, method_router);
            let path_segments = string_to_path_segments(path);
            for params in vec_params {
                // path_segments
            }
            ROUTES_TO_AXUM.lock().unwrap().insert(path.to_owned());
        }

        router
    }
}

static ROUTES_TO_AXUM: LazyLock<Arc<Mutex<HashSet<String>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashSet::new())));

pub async fn render_routes_to_files() {
    std::fs::create_dir_all("dist/").unwrap();
    let mut static_assets = Vec::new();

    for route in &*ROUTES_TO_AXUM.lock().unwrap() {
        let resp_text = reqwest::get(format!("http://localhost:{}{route}", *PORT))
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

        let mut path = PathBuf::new();
        path.push("dist");
        path.extend(route.split('/'));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
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

    copy_dir_all("public", "dist").unwrap();
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

fn string_to_path_segments<'l>(s: &'l str) -> Vec<PathSegment<'l>> {
    s.split('/')
        .skip(1)
        .map(|segment| {
            match segment
                .chars()
                .nth(1)
                .unwrap_or_else(|| panic!("Invalid path: {s}"))
            {
                '*' => PathSegment::Wildcard,
                ':' | '{' => PathSegment::Param,
                _ => PathSegment::String(segment),
            }
        })
        .collect()
}
