use {
    axum::{extract::Request, response::Response},
    derive_builder::Builder,
    std::{
        borrow::Cow,
        collections::{HashMap, HashSet},
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
    tower::{Layer, Service},
};

pub type Tags = HashMap<&'static str, Cow<'static, str>>;
pub type Files = HashSet<Cow<'static, str>>;

#[derive(Clone, Builder)]
pub struct MetaLayer {
    /// Corresponds to the default meta tags in case the page rendered doesn't have them
    ///
    /// # Example
    /// To add a default meta description if the page doesn't have one
    /// ```
    /// use {
    ///     PROJECT_NAME_TO_RESOLVE::shared::wini::layer::MetaLayerBuilder,
    ///     std::collections::HashMap,
    /// };
    ///
    /// MetaLayerBuilder::default()
    ///     .default_meta(HashMap::from([("description", "Hello world!".into())]))
    ///     .build();
    /// ```
    #[builder(default)]
    default_meta: Tags,
    /// Will always render these meta tags, regardless of what the page sends as meta tags
    ///
    /// # Example
    /// To always send "Hello world!" as the meta description
    /// ```
    /// use {
    ///     PROJECT_NAME_TO_RESOLVE::shared::wini::layer::MetaLayerBuilder,
    ///     std::collections::HashMap,
    /// };
    ///
    /// MetaLayerBuilder::default()
    ///     .force_meta(HashMap::from([("description", "Hello world!".into())]))
    ///     .build();
    /// ```
    #[builder(default)]
    force_meta: Tags,
}

impl<S> Layer<S> for MetaLayer {
    type Service = MetaService<S>;

    fn layer(&self, service: S) -> Self::Service {
        MetaService {
            inner: service,
            default_meta: Arc::new(self.default_meta.clone()),
            force_meta: Arc::new(self.force_meta.clone()),
        }
    }
}

#[derive(Clone)]
pub struct MetaService<S> {
    inner: S,
    default_meta: Arc<Tags>,
    force_meta: Arc<Tags>,
}

impl<S> Service<Request> for MetaService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static + Send>>;
    type Response = S::Response;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let fut = self.inner.call(req);

        let default_meta = Arc::clone(&self.default_meta);
        let force_meta = Arc::clone(&self.force_meta);

        Box::pin(async move {
            let resp: Response = fut.await?;

            let (mut resp_parts, resp_body) = resp.into_parts();

            {
                if let Some(extensions) = resp_parts.extensions.get_mut::<Tags>() {
                    for (tag, value) in &*force_meta {
                        extensions.insert(
                            tag,
                            match value {
                                Cow::Owned(string) => Cow::Owned(string.to_owned()),
                                Cow::Borrowed(str) => Cow::Borrowed(str),
                            },
                        );
                    }

                    for (tag, value) in &*default_meta {
                        if !extensions.contains_key(tag) {
                            extensions.insert(
                                tag,
                                match value {
                                    Cow::Owned(string) => Cow::Owned(string.to_owned()),
                                    Cow::Borrowed(str) => Cow::Borrowed(str),
                                },
                            );
                        }
                    }
                } else {
                    let mut tags: Tags = HashMap::new();

                    for (tag, value) in &*force_meta {
                        tags.insert(
                            tag,
                            match value {
                                Cow::Owned(string) => Cow::Owned(string.to_owned()),
                                Cow::Borrowed(str) => Cow::Borrowed(*str),
                            },
                        );
                    }

                    for (tag, value) in &*default_meta {
                        if !tags.contains_key(tag) {
                            tags.insert(
                                tag,
                                match value {
                                    Cow::Owned(string) => Cow::Owned(string.to_owned()),
                                    Cow::Borrowed(str) => Cow::Borrowed(*str),
                                },
                            );
                        }
                    }

                    resp_parts.extensions.insert(tags);
                }
            }

            Ok(Response::from_parts(resp_parts, resp_body))
        })
    }
}
