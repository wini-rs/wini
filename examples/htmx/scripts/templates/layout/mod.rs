use {
    maud::{html, MarkUp, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(child: &str) -> MarkUp {
    html! {(PreEscaped(child))}
}
