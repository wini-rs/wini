use {
    crate::shared::wini::err::ServerResult,
    maud::{html, Markup, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(s: Markup) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
        (s)
    })
}
// IFFEAT test
use {
    axum::{body::Body, http::response::Parts},
    hyper::StatusCode,
};
#[layout]
pub async fn mut_parts(_: Parts) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
    })
}
#[layout]
pub async fn parts(_: Parts) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
    })
}
#[layout]
pub async fn parts_and_body(_: Parts, _body: Body) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
    })
}
#[layout]
pub async fn status_code(status_code: StatusCode) -> Markup {
    match status_code {
        StatusCode::OK => html! {"ok!"},
        StatusCode::NOT_FOUND => html! {"404"},
        _ => html! {"Other status code"},
    }
}

#[layout]
pub async fn err_backtrace_logging(status_code: StatusCode) -> Markup {
    match status_code {
        StatusCode::OK => html! {"ok!"},
        StatusCode::NOT_FOUND => html! {"404"},
        _ => html! {"Other status code"},
    }
}
// ENDIF
