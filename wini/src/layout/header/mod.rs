use {
    maud::{html, Markup, PreEscaped},
    wini_macros::wrapper,
};

#[wrapper]
pub async fn render(s: &str) -> Markup {
    html! {
        header {
            "Welcome to Wini!"
        }
        (PreEscaped(s))
    }
}
