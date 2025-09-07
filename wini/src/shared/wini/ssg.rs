use {
    axum::{handler::Handler, routing::MethodRouter, Router},
    std::{
        collections::{HashMap, HashSet},
        fs::create_dir_all,
        sync::{Arc, LazyLock, Mutex},
    },
};

/// A basic router for SSG
#[derive(Debug, Default)]
pub(crate) struct SsgRouter<'l> {
    routes: HashMap<&'l str, MethodRouter<()>>,
}

impl<'l> SsgRouter<'l> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn route(mut self, path: &'l str, m: MethodRouter<()>) -> Self {
        self.routes.insert(path, m);
        self
    }

    pub fn route_with_params(mut self, path: &'l str, m: MethodRouter<()>) -> Self {
        self.routes.insert(path, m);
        self
    }

    pub fn into_axum_router(self) -> Router {
        let mut router = Router::new();

        for (path, method_router) in self.routes {
            router = router.route(path, method_router);
        }

        router
    }
}

static ROUTES_TO_AXUM: LazyLock<Arc<Mutex<HashSet<String>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashSet::new())));

pub async fn render_routes_to_files() {
    std::fs::create_dir_all("public/ssg-out");
    let mut path_to_uuid = HashMap::new();
    for route in &*ROUTES_TO_AXUM.lock().unwrap() {
        let resp_text = reqwest::get(route).await.unwrap().text().await.unwrap();
        let filename = uuid::Uuid::new();
        std::fs::write("public/ssg-out", contents).expect("Couldn't write the ")
    }
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
            match segment.chars().nth(1).expect(&format!("Invalid path: {s}")) {
                '*' => PathSegment::Wildcard,
                ':' | '{' => PathSegment::Param,
                _ => PathSegment::String(segment),
            }
        })
        .collect()
}
