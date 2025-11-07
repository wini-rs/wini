use {
    maud::{html, MarkUp, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(child: Markup) -> MarkUp {
    html! {(child)}
}
