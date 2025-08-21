use {
    cached::proc_macro::cached,
    maud::{Markup, html},
    wini_macros::{init_cache, page},
};

#[init_cache]
#[page]
#[cached]
pub async fn render() -> Markup {
    html! {
        button #hello {
            "Say hello!"
        }
    }
}

// IFFEAT test
use crate::{components::button, shared::wini::err::ServerResult};

#[page]
pub async fn test_button() -> ServerResult<Markup> {
    Ok(html! {
        (button().await?)
        button #hello {
            "Say hello!"
        }
    })
}

#[page(title = "hello world", keywords = ["hello", "world"], other_meta = ["hello" = "world", "world" = "hello"])]
pub async fn test_meta() -> Markup {
    html! {
        button #hello {
            "Say hello!"
        }
    }
}

// ENDIF
