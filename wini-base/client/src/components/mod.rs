// IFFEAT test
use {
    crate::shared::wini::err::{ServerErrorKind, ServerResult},
    cached::proc_macro::cached,
    hyper::StatusCode,
    maud::{Markup, html},
    wini_macros::component,
};

#[cached]
#[component]
pub async fn button() -> Markup {
    html! {
        button {
            "Welcome to Wini!"
        }
    }
}

#[component]
pub async fn err_component1() -> ServerResult<Markup> {
    Err(ServerErrorKind::Status(StatusCode::NOT_FOUND).into())
}

#[component]
pub async fn err_component2() -> ServerResult<Markup> {
    Ok(html! {
        [err_component1?]
    })
}

#[component(js_pkgs = [])]
pub async fn js_pkgs() -> ServerResult<Markup> {
    Ok(html! {})
}
// ENDIF
