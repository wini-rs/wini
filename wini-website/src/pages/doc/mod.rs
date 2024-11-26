use {
    axum::extract::Request,
    font_awesome_as_a_crate::{svg, Type},
    maud::{html, Markup, PreEscaped},
    std::{collections::HashMap, path::Path, sync::LazyLock},
    wini_macros::page,
};

// static MARKDOWN_PAGES: LazyLock<HashMap<String, String>> = LazyLock::new(|| Vec::new());


#[derive(Debug, serde::Deserialize)]
enum PageOrDirectory<'l> {
    Page {
        title: &'l str,
        page: &'l str,
    },
    Directory {
        is_ordered: bool,
        title: &'l str,
        page: Option<&'l str>,
        pages: Vec<PageOrDirectory<'l>>,
    },
}

#[derive(Debug)]
enum VecOrStr<'l> {
    Vec(Vec<&'l str>),
    Str(&'l str),
}

impl<'l> PageOrDirectory<'l> {
    fn rec_get_pages(&self) -> VecOrStr {
        match self {
            PageOrDirectory::Page { page, .. } => VecOrStr::Str(page),
            PageOrDirectory::Directory { pages, page, .. } => {
                let mut final_pages = page.map_or_else(Default::default, |p| vec![p]);
                for page in pages {
                    match page.rec_get_pages() {
                        VecOrStr::Str(s) => final_pages.push(s),
                        VecOrStr::Vec(v) => final_pages.extend(v),
                    }
                }
                VecOrStr::Vec(final_pages)
            },
        }
    }

    fn rec_display(&self) -> Markup {
        match self {
            PageOrDirectory::Page { title, page } => {
                html! { li { a href=(format!("/doc/{page}")) { (title) }}}
            },
            PageOrDirectory::Directory {
                pages, page, title, ..
            } => {
                html! {
                    @if let Some(page) = page {
                        li { a href=(format!("/doc/{page}")) {(title) }}
                    } @else {
                        li { (title) }
                    }
                    ol {
                        @for page in pages {
                            (page.rec_display())
                        }
                    }
                }
            },
        }
    }

    fn get_nearest_pages(&self, page: &str) -> (Option<String>, Option<String>) {
        let pages = self.rec_get_pages();

        if let VecOrStr::Vec(v) = pages {
            if let Some(index_at_page) = v.iter().position(|p| *p == page) {
                if index_at_page == 0 {
                    (None, v.get(index_at_page + 1).map(|e| (*e).to_owned()))
                } else {
                    (
                        v.get(index_at_page - 1).map(|e| (*e).to_owned()),
                        v.get(index_at_page + 1).map(|e| (*e).to_owned()),
                    )
                }
            } else {
                (None, None)
            }
        } else {
            (None, None)
        }
    }
}

fn search_file_recursively(dir: &str, target_name: &str) -> std::io::Result<Option<String>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(found) = search_file_recursively(path.to_str().unwrap(), target_name)? {
                return Ok(Some(found));
            }
        } else if path.is_file() && path.file_name().map_or(false, |name| name == target_name) {
            return Ok(Some(std::fs::read_to_string(path).unwrap()));
        }
    }

    Ok(None)
}

pub fn pages() -> HashMap<String, String> {
    let page_structure: PageOrDirectory = ron::from_str(&include_str!("./structure.ron")).unwrap();
    match page_structure.rec_get_pages() {
        VecOrStr::Vec(v) => {
            v.iter()
                .map(|page| {
                    let file_content = search_file_recursively(".".into(), &format!("{page}.md"))
                        .unwrap()
                        .unwrap();

                    let parser = pulldown_cmark::Parser::new(&file_content);
                    let mut html_output = String::new();
                    pulldown_cmark::html::push_html(&mut html_output, parser);

                    ((*page).to_owned(), html_output)
                })
                .collect()
        },
        VecOrStr::Str(_) => panic!("Should not occur"),
    }
}

static PAGES_STRUCTURE: LazyLock<PageOrDirectory> =
    LazyLock::new(|| ron::from_str(&include_str!("./structure.ron")).unwrap());



#[page]
pub async fn render(req: Request) -> Markup {
    let pages = pages();
    let requested_page = req.uri().path().split('/').last().unwrap();
    let result = pages.get(requested_page).unwrap();

    let (previous_page, next_page) = PAGES_STRUCTURE.get_nearest_pages(requested_page);

    html! {
        nav #sidebar {
            (PAGES_STRUCTURE.rec_display())
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
                @if let Some(previous_page) = previous_page {
                    a href={"/doc/" (previous_page)} .previous-next {
                        (PreEscaped(
                                svg(Type::Solid, "angle-left"

                                ).unwrap()))
                    }
                }
                @else {
                    div .placeholder-previous-next {}
                }
                main {
                    #content {
                        (PreEscaped(result))
                    }
                }
                @if let Some(next_page) = next_page {
                    a href={"/doc/" (next_page)} .previous-next {
                        (PreEscaped(
                            svg(
                                Type::Solid,
                                "angle-right"
                            )
                            .unwrap()
                        ))
                    }
                } @else {
                    .placeholder-previous-next {}
                }
            }
        }
    }
}