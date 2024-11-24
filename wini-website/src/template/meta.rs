use {
    axum::http::response::Parts,
    maud::Markup,
    std::{collections::HashMap, sync::LazyLock},
};


pub static META_MAPPINGS: LazyLock<HashMap<&'static str, Vec<&'static str>>> =
    LazyLock::new(|| {
        HashMap::from([
            ("title", vec!["og:title"]),
            ("description", vec!["description", "og:description"]),
            ("keywords", vec!["keywords"]),
            ("robots", vec!["robots"]),
            ("author", vec!["author"]),
            ("site_name", vec!["og:site_name"]),
            ("lang", vec!["language"]),
            ("img", vec!["og:image"]),
        ])
    });

pub fn add_meta_tags(res_parts: &mut Parts) -> Markup {
    let meta_tags = res_parts
        .headers
        .iter()
        .filter(|(header_name, _)| header_name.as_str().starts_with("meta-"))
        .map(|(head_name, _)| head_name.as_str().to_string())
        .collect::<Vec<String>>();

    let html = maud::html! {
        @if let Some(title) = res_parts.headers.get("meta-title") {
            title { (title.to_str().unwrap_or_default()) }
        }
        @for (tag_name, tag_value) in meta_tags
            .into_iter()
            .map(|tag_name| (res_parts.headers.remove(&tag_name).expect("Already matched."), tag_name))
            .flat_map(|(tag_value, tag_name)|
                match (*META_MAPPINGS).get(&tag_name[5..]) {
                    Some(names) => names.iter().map(|name| ((*name).to_owned(), tag_value.to_owned())).collect(),
                    None => vec![(tag_name, tag_value)],
                }
            ) {
            meta name=(tag_name) content=(tag_value.to_str().unwrap_or_default());
        }
    };

    html
}
