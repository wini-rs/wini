use {
    crate::shared::wini::PORT,
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

/// A basic router for SSG
#[derive(Debug, Default)]
pub(crate) struct SsgRouter<'l> {
    routes: HashMap<&'l str, (MethodRouter<()>, Option<Vec<Vec<Cow<'l, str>>>>)>,
}

impl<'l> SsgRouter<'l> {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(unused, reason = "Not necessarily used")]
    pub fn route(mut self, path: &'l str, m: MethodRouter<()>) -> Self {
        self.routes.insert(path, (m, None));
        self
    }

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
