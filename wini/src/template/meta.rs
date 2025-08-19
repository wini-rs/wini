use {
    crate::shared::wini::layer::Tags,
    axum::http::response::Parts,
    maud::{Markup, html},
    std::{collections::HashMap, sync::LazyLock},
};


pub static META_MAPPINGS: LazyLock<HashMap<&'static str, Vec<&'static str>>> =
    LazyLock::new(|| {
        HashMap::from([
            ("title", vec!["og:title"]),
            ("description", vec!["description", "og:description"]),
            // ("keywords", vec!["keywords"]),
            // ("robots", vec!["robots"]),
            // ("author", vec!["author"]),
            ("site_name", vec!["og:site_name"]),
            ("lang", vec!["language"]),
            ("img", vec!["og:image"]),
        ])
    });

pub fn add_meta_tags(res_parts: &mut Parts) -> Markup {
    if let Some(meta_tags) = res_parts.extensions.get::<Tags>() {
        html! {
            @if let Some(title) = meta_tags.get("title") {
                title { (title) }
            }
            @for (tag_name, tag_value) in meta_tags {
                @if let Some(names) = META_MAPPINGS.get(tag_name) {
                    @for name in names {
                        meta name=(name) content=(tag_value);
                    }
                } @else {
                    meta name=(tag_name) content=(tag_value);
                }
            }
        }
    } else {
        html!()
    }
}
