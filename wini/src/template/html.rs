use maud::{Markup, PreEscaped, DOCTYPE};

pub fn html(
    s: &str,
    scripts_files: Vec<String>,
    style_sheets: Vec<String>,
    meta: Markup,
) -> String {
    maud::html! {
        (DOCTYPE)
        html {
            head {
                style { (include_str!("./always_loaded.css").trim_end()) }
                @for style_sheet in style_sheets {
                    link rel="stylesheet" href=(style_sheet);
                }
                link rel="icon" href="/favicon.ico" sizes="any";
                link rel="icon" href="/favicon.svg" type="image/svg+xml";
                link rel="stylesheet" href="/main.css";
                script src="/helpers.min.js" defer {}
                @for script in scripts_files {
                    script src=(script) defer {}
                }
                (meta)
            }
            body {
                (PreEscaped(s))
            }
        }
    }
    .into_string()
}
