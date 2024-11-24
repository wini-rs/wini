use {
    font_awesome_as_a_crate::{svg, Type},
    maud::{html, Markup, PreEscaped},
    wini_macros::wrapper,
};

#[wrapper]
pub async fn render(child: &str) -> Markup {
    html! {
        nav #sidebar {
            ol {
                li {
                    "hey"
                }
            }
        }
        main {
            header {
                div {
                    button #hide-sidebar {
                        img src="/bars-solid.svg";
                    }
                }
                h1 {"Wini's book"}
                div {
                    a href="https://github.com/wini-rs/wini" {
                        img src="/github.svg";
                    }
                    a href="https://codeberg.org/wini/wini" {
                        img src="/codeberg.svg";
                    }
                }
            }
            div #horizontal-content {
                button .previous-next {
                    (PreEscaped(
                    svg(Type::Solid, "angle-left"

                    ).unwrap()))
                }
                main {
                    (PreEscaped(child))

                }
                button .previous-next {
                    (PreEscaped(
                    svg(Type::Solid, "angle-right"

                    ).unwrap()))
                }
            }
        }
    }
}
