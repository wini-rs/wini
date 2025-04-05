use {
    std::sync::LazyLock,
    syntect::{
        highlighting::ThemeSet,
        html::highlighted_html_for_string,
        parsing::{SyntaxSet, SyntaxSetBuilder},
    },
    tl::Node,
};

pub fn style_code(code: &Node, parser: &tl::Parser<'_>) -> String {
    let theme = &THEMES.themes["base16-eighties.dark"];

    let raw_code = code.inner_html(parser);
    let Some(language) = code
        .as_tag()
        .and_then(|e| e.attributes().get("class"))
        .flatten()
        .map(|class| {
            let language = String::from_utf8(class.as_bytes().to_vec()).unwrap();
            language.trim_start_matches("language-").to_owned()
        })
    else {
        return format!("<pre><code>{raw_code}</code></pre>");
    };
    let unescaped_code = &raw_code
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&amp;", "&");



    let html = if let Some(set) = CUSTOM_SYNTAXES.find_syntax_by_extension(&language) {
        highlighted_html_for_string(&unescaped_code, &CUSTOM_SYNTAXES, &set, theme).unwrap()
    } else {
        let set = NORMAL_SYNTAXES.find_syntax_by_extension(&language).unwrap();
        highlighted_html_for_string(&unescaped_code, &NORMAL_SYNTAXES, &set, theme).unwrap()
    };

    html
}

static NORMAL_SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEMES: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);
static CUSTOM_SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(custom_syntaxes);

fn custom_syntaxes() -> SyntaxSet {
    let mut ss = SyntaxSetBuilder::new();
    ss.add_from_folder(".", true).unwrap();
    ss.build()
}
