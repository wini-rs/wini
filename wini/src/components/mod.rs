// IFFEAT test
use {
    crate::shared::wini::err::ServerResult,
    maud::{Markup, html},
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
// ENDIF
