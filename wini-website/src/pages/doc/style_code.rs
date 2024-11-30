use {
    syntect::{
        highlighting::ThemeSet,
        html::highlighted_html_for_string,
        parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder},
    },
    tl::Node,
};

pub fn style_code(code: &Node, parser: &tl::Parser<'_>) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-eighties.dark"];

    let raw_code = code.inner_html(parser);
    println!("{raw_code}");
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


    let custom_syntaxes = custom_syntaxes();
    let c = custom_syntaxes.clone();

    // let (ss, set) = .map_or_else(
    //     move || (ss, ss.find_syntax_by_extension(&language).unwrap()),
    //     |syntax| (custom_syntaxes, syntax),
    // );

    let html = if let Some(set) = custom_syntaxes.find_syntax_by_extension(&language) {
        highlighted_html_for_string(&unescaped_code, &custom_syntaxes, &set, theme).unwrap()
    } else {
        let set = ss.find_syntax_by_extension(&language).unwrap();
        highlighted_html_for_string(&unescaped_code, &ss, &set, theme).unwrap()
    };

    html
}

fn custom_syntaxes() -> SyntaxSet {
    let mut ss = SyntaxSetBuilder::new();
    ss.add_from_folder(".", true);
    ss.build()
}
