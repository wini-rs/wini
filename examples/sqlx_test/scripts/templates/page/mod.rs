use {
    maud::{html, Markup},
    wini_macros::page,
};

#[page]
pub async fn render() -> Markup {
    html! {}
}
