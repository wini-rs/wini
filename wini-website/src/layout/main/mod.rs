use {
    font_awesome_as_a_crate::{svg, Type},
    maud::{html, Markup, PreEscaped},
    wini_macros::wrapper,
};

#[wrapper]
pub async fn render(child: &str) -> Markup {
    html! {(PreEscaped(child))}
}
