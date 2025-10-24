use {
    crate::{
        shared::wini::err::{Backtrace, ServerError},
        utils::wini::buffer::buffer_to_string,
    },
    axum::{
        body::Body,
        http::{response, Extensions},
        response::IntoResponse,
    },
    hyper::{HeaderMap, StatusCode, Version},
    maud::{Markup, PreEscaped},
    std::convert::Infallible,
};

/// Similar to [`axum::extract::FromRequestParts`] but for [`axum::response::Response`]s.
/// Used in `#[layout]`s
pub trait FromResponseParts<S>: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    fn from_response_parts(
        parts: &mut response::Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

/// Similar to [`axum::extract::FromRequest`] but for [`axum::response::Response`]s.
/// Used in `#[layout]`s
pub trait FromResponseBody<S>: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    fn from_response_body(
        resp: Body,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

/////////////////////////////
// Basic FromResponseParts //
/////////////////////////////
impl<S> FromResponseParts<S> for StatusCode
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_response_parts(
        parts: &mut response::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts.status)
    }
}

impl<S> FromResponseParts<S> for response::Parts
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_response_parts(
        parts: &mut response::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts.clone())
    }
}

impl<S> FromResponseParts<S> for Extensions
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_response_parts(
        parts: &mut response::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts.extensions.clone())
    }
}

impl<S> FromResponseParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_response_parts(
        parts: &mut response::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts.version)
    }
}

impl<S> FromResponseParts<S> for HeaderMap
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_response_parts(
        parts: &mut response::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts.headers.clone())
    }
}

impl<S> FromResponseParts<S> for Option<Backtrace>
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_response_parts(
        parts: &mut response::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts.extensions.get().cloned())
    }
}

////////////////////////////
// Basic FromResponseBody //
////////////////////////////
impl<S> FromResponseBody<S> for Markup
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_response_body(body: Body, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Markup {
            content: PreEscaped(buffer_to_string(body).await?),
            linked_files: Default::default(),
        })
    }
}

impl<S> FromResponseBody<S> for Body
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_response_body(body: Body, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(body)
    }
}
