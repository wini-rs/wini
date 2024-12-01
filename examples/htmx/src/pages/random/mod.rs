use {
    maud::{html, Markup},
    random_word::Lang,
    wini_macros::page,
};

#[page]
pub async fn render() -> Markup {
    html! {
        span { (random_word::gen(Lang::En)) }
    }
}
