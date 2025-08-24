use {
    cached::proc_macro::cached,
    maud::{Markup, html},
    wini_macros::{init_cache, page},
};

#[init_cache]
#[page]
#[cached]
pub async fn render() -> Markup {
    html! {
        button #hello {
            "Say hello!"
        }
    }
}

// IFFEAT test
use crate::{components::button, shared::wini::err::ServerResult};

#[page]
pub async fn test_button() -> ServerResult<Markup> {
    Ok(html! {
        (button().await?)
        button #hello {
            "Say hello!"
        }
    })
}

#[page(title = "hello world", keywords = ["hello", "world"], other_meta = ["hello" = "world", "world" = "hello"])]
pub async fn test_meta() -> Markup {
    html! {
        button #hello {
            "Say hello!"
        }
    }
}


#[tokio::test]
async fn test_meta_page() {
    use {
        crate::template,
        axum::{Router, middleware::from_fn, routing::get},
        axum_test::TestServer,
    };

    let app = Router::new()
        .route("/meta", get(test_meta))
        .layer(from_fn(template::template));
    let server = TestServer::new(app).expect("creates a server");

    let resp = server.get("/meta").await;

    resp.assert_status_ok();
    let txt = resp.text();
    // Title
    assert!(txt.contains("<title>hello world</title>"));
    assert!(txt.contains(r#"<meta property="og:title" content="hello world">"#));
    // Keywords
    assert!(txt.contains(r#"<meta name="keywords" content="hello, world">"#));
    // Other meta
    assert!(txt.contains(r#"<meta name="world" content="hello">"#));
    assert!(txt.contains(r#"<meta name="hello" content="world">"#));
}
// ENDIF
