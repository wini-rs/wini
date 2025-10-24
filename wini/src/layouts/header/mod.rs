use {
    crate::shared::wini::err::ServerResult,
    hyper::{HeaderMap, Uri},
    maud::{html, Markup},
    wini_macros::layout,
};

#[layout]
pub async fn render(#[from_request_parts] _h: HeaderMap, s: Markup) -> ServerResult<Markup> {
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
pub async fn status_code_with_markup(status_code: StatusCode, markup: Markup) -> Markup {
    if status_code.is_success() {
        markup
    } else {
        html! {
            h1 .error {
                "Oops! An error occurred!"
            }
        }
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

// From request
#[layout]
pub async fn uri(uri: Uri) -> ServerResult<Markup> {
    Ok(html! {
        header {
            (uri)
        }
    })
}
// ENDIF
