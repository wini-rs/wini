use {
    maud::{html, PreEscaped},
    wini_macros::wrapper,
};

#[wrapper]
pub async fn render(child: &str) -> String {
    html! {}.into_string()
}
