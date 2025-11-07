//! Traits helpers used to know if a type implements a trait.
//! Used in `#[layout]`s

use {
    crate::shared::wini::response::{FromResponseBody, FromResponseParts},
    axum::{
        body::Body,
        extract::FromRequestParts,
        http::{request::Parts as RequestParts, response::Parts as ResponseParts},
        response::IntoResponse,
    },
    std::{convert::Infallible, pin::Pin},
};


/// Check if it's [`axum::extract::FromRequestParts`]
pub trait IsFromRequestParts {
    const IS_FROM_REQUEST_PARTS: bool = false;
}

impl<T> IsFromRequestParts for T {
    default const IS_FROM_REQUEST_PARTS: bool = false;
}

impl<T: FromRequestParts<()>> IsFromRequestParts for T {
    const IS_FROM_REQUEST_PARTS: bool = true;
}


/// Check if it's [`crate::shared::wini::response::FromResponseBody`]
pub trait IsFromResponseBody {
    const IS_FROM_RESPONSE_BODY: bool = false;
}

impl<T> IsFromResponseBody for T {
    default const IS_FROM_RESPONSE_BODY: bool = false;
}

impl<T: FromResponseBody<()>> IsFromResponseBody for T {
    const IS_FROM_RESPONSE_BODY: bool = true;
}


/// Check if it's [`crate::shared::wini::response::FromResponseParts`]
pub trait IsFromResponseParts {
    const IS_FROM_RESPONSE_PARTS: bool = false;
}

impl<T> IsFromResponseParts for T {
    default const IS_FROM_RESPONSE_PARTS: bool = false;
}

impl<T: FromResponseParts<()>> IsFromResponseParts for T {
    const IS_FROM_RESPONSE_PARTS: bool = true;
}


/// **⚠️ INTERNAL USE ONLY – DO NOT USE DIRECTLY**
///
/// This trait is only used internally by the `#[layout]` proc macro to execute
/// code based on whether a type implements a trait.
///
/// **Using this trait directly will result in runtime panics.**
/// Use the appropriate `From*` traits instead.
#[doc(hidden)]
pub trait FromRequestPartsHelper<S> {
    type RejectionHelper: IntoResponse;

    fn __from_request_parts<'a>(
        _parts: &'a mut RequestParts,
        _state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        unreachable!()
    }
}

impl<T, S> FromRequestPartsHelper<S> for T {
    default type RejectionHelper = Infallible;

    default fn __from_request_parts<'a>(
        _parts: &'a mut RequestParts,
        _state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        Box::pin(async { unreachable!() })
    }
}

impl<T: FromRequestParts<S>, S> FromRequestPartsHelper<S> for T {
    type RejectionHelper = T::Rejection;

    fn __from_request_parts<'a>(
        parts: &'a mut RequestParts,
        state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        Box::pin(T::from_request_parts(parts, state))
    }
}

/// **⚠️ INTERNAL USE ONLY – DO NOT USE DIRECTLY**
///
/// This trait is only used internally by the `#[layout]` proc macro to execute
/// code based on whether a type implements a trait.
///
/// **Using this trait directly will result in runtime panics.**
/// Use the appropriate `From*` traits instead.
#[doc(hidden)]
pub trait FromResponseBodyHelper<S> {
    type RejectionHelper: IntoResponse;

    fn __from_response_body<'a>(
        _body: Body,
        _state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a;
}

impl<T, S> FromResponseBodyHelper<S> for T {
    default type RejectionHelper = Infallible;

    default fn __from_response_body<'a>(
        _body: Body,
        _state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        Box::pin(async { unreachable!() })
    }
}

impl<T: FromResponseBody<S>, S> FromResponseBodyHelper<S> for T {
    type RejectionHelper = T::Rejection;

    fn __from_response_body<'a>(
        body: Body,
        state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        Box::pin(T::from_response_body(body, state))
    }
}


/// **⚠️ INTERNAL USE ONLY – DO NOT USE DIRECTLY**
///
/// This trait is only used internally by the `#[layout]` proc macro to execute
/// code based on whether a type implements a trait.
///
/// **Using this trait directly will result in runtime panics.**
/// Use the appropriate `From*` traits instead.
#[doc(hidden)]
pub trait FromResponsePartsHelper<S> {
    type RejectionHelper: IntoResponse;

    fn __from_response_parts<'a>(
        _parts: &'a mut ResponseParts,
        _state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        unreachable!()
    }
}

impl<T, S> FromResponsePartsHelper<S> for T {
    default type RejectionHelper = Infallible;

    default fn __from_response_parts<'a>(
        _parts: &'a mut ResponseParts,
        _state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        Box::pin(async { unreachable!() })
    }
}

impl<T: FromResponseParts<S>, S> FromResponsePartsHelper<S> for T {
    type RejectionHelper = T::Rejection;

    fn __from_response_parts<'a>(
        parts: &'a mut ResponseParts,
        state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::RejectionHelper>> + 'a + Send>>
    where
        Self: Sized + 'a,
    {
        Box::pin(T::from_response_parts(parts, state))
    }
}
