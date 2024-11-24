use {
    maud::{html, Markup},
    wini_macros::{cache, page},
};

#[cache]
#[page]
pub async fn render() -> Markup {
    html! {
        button #hello {
            "Say hello!"
        }
    }
}
