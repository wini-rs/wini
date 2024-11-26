use {
    maud::{html, MarkUp, PreEscaped},
    wini_macros::wrapper,
};

#[wrapper]
pub async fn render(child: &str) -> MarkUp {
    html! {(PreEscaped(child))}
}
