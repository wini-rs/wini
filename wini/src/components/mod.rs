// IFFEAT test
use {
    crate::shared::wini::err::ServerResult,
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
// ENDIF
