use {
    crate::shared::wini::err::ServerResult,
    maud::{Markup, html},
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
    crate::shared::wini::err::Backtrace,
    axum::{body::Body, http::response::Parts},
    hyper::{HeaderMap, StatusCode, Uri},
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

// This is a test for `FromRequestParts`
#[layout]
pub async fn uri(uri: Uri) -> ServerResult<Markup> {
    Ok(html! {
        header {
            (uri)
        }
    })
}

#[layout]
pub async fn remove_doubt(
    #[from_request_parts] _headers_req: HeaderMap,
    #[from_response_parts] _headers_resp: HeaderMap,
    s: Markup,
) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
        (s)
    })
}

#[layout]
pub async fn handle_error_doubt(
    #[from_request_parts] _headers_req: HeaderMap,
    _backtrace: Option<Backtrace>,
    s: Markup,
) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
        (s)
    })
}
// ENDIF
