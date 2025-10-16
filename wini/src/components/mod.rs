// IFFEAT test
use {
    crate::shared::wini::err::{ServerErrorKind, ServerResult},
    hyper::StatusCode,
    maud::{html, Markup},
    wini_macros::component,
};

#[component]
pub async fn button() -> ServerResult<Markup> {
    Ok(html! {
        button {
            "Welcome to Wini!"
        }
    })
}

#[component]
pub async fn err_component1() -> ServerResult<Markup> {
    Err(ServerErrorKind::Status(StatusCode::NOT_FOUND).into())
}

#[component]
pub async fn err_component2() -> ServerResult<Markup> {
    Ok(html! {
        [err_component1]
    })
}
// ENDIF
