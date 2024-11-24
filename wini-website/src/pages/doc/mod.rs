use {
    axum::extract::Request,
    maud::{html, Markup},
    std::{collections::HashMap, sync::LazyLock},
    wini_macros::page,
};

// static MARKDOWN_PAGES: LazyLock<HashMap<String, String>> = LazyLock::new(|| Vec::new());

static PAGES_STRUCTURE: LazyLock<PageOrDirectory> =
    LazyLock::new(|| PageOrDirectory::Directory(vec![PageOrDirectory::Page("help.md")]));

enum PageOrDirectory {
    Page(&'static str),
    Directory(Vec<PageOrDirectory>),
}



#[page]
pub async fn render(req: Request) -> Markup {
    println!("{:#?}", req);
    println!("{:#?}", req.uri());
    html! {
        "hey"
    }
}
