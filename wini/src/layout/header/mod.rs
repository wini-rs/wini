use {
    maud::{html, Markup, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(s: &str) -> Markup {
    html! {
        header {
            "Welcome to Wini!"
        }
        (PreEscaped(s))
    }
}
