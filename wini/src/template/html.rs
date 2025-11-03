use {
    maud::{Markup, PreEscaped, DOCTYPE},
    std::borrow::Cow,
};

pub fn html(
    s: &str,
    scripts_files: Vec<Cow<str>>,
    style_sheets: Vec<Cow<str>>,
    meta: &Markup,
) -> String {
    maud::html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                (meta)

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
            }
            body {
                (PreEscaped(s))
            }
        }
    }
    .into_string()
}
