use {
    axum::{extract::Request, response::Response},
    derive_builder::Builder,
    std::{
        borrow::Cow,
        collections::{HashMap, hash_map},
        pin::Pin,
        task::{Context, Poll},
    },
    tower::{Layer, Service},
};


#[derive(Clone, Builder)]
pub struct MetaLayer {
    /// Corresponds to the default meta tags in case the page rendered doesn't have them
    ///
    /// # Example
    /// To add a default meta description if the page doesn't have one
    /// ```
    /// MetaLayerBuilder::default()
    ///     .default_meta(HashMap::from(["description".into(), "Hello world!".into()]))
    ///     .build()
    /// ```
    default_meta: HashMap<Cow<'static, str>, Cow<'static, str>>,
    /// Will always render theses meta tags, whatever the page sends has meta tags
    ///
    /// # Example
    /// To always send "Hello world!" as the meta description
    /// ```
    /// MetaLayerBuilder::default()
    ///     .default_meta(HashMap::from(["description".into(), "Hello world!".into()]))
    ///     .build()
    /// ```
    force_meta: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl<S> Layer<S> for MetaLayer {
    type Service = MetaService<S>;

    fn layer(&self, service: S) -> Self::Service {
        MetaService {
            inner: service,
            default_meta: self.default_meta.clone(),
            force_meta: self.force_meta.clone(),
        }
    }
}

#[derive(Clone)]
pub struct MetaService<S> {
    inner: S,
    default_meta: HashMap<Cow<'static, str>, Cow<'static, str>>,
    force_meta: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl<S> Service<Request> for MetaService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Response = S::Response;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let fut = self.inner.call(req);
        Box::pin(async move {
            let response: Response = fut.await?;
            // TODO
            Ok(response)
        })
    }
}
