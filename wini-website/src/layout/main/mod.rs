use {
    crate::{hs, pages::doc::PAGES_STRUCTURE},
    maud::{html, Markup, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(child: &str) -> Markup {
    html! {
        script type="text/hyperscript" {
            (hs!(
                def liClick()
                    if the innerWidth of the window < 1200 then
                        add .hidden to #sidebar
                    end
                end
            ))
        }
        nav #sidebar {
            (PAGES_STRUCTURE.rec_display())
        }
        main {
            header {
                div {
                    button #hide-sidebar _="on click toggle .hidden on #sidebar" {
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
            div #horizontal-content hx-disinherit="*" {
                (PreEscaped(child))
            }
        }
    }
}
