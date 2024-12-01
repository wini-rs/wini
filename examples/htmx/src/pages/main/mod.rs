use {
    maud::{html, Markup},
    wini_macros::{cache, page},
};

#[cache]
#[page]
pub async fn render() -> Markup {
    html! {
        button #hello  hx-get="/random" hx-target="#word-content" {
            "Generate a random word:"
        }
        main #word-content {}
    }
}
