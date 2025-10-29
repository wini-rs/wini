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
use crate::{
    components::{button, err_component2},
    shared::wini::err::ServerResult,
};

#[page]
pub async fn test_button() -> ServerResult<Markup> {
    Ok(html! {
        [button]
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

#[page]
pub async fn err_page() -> ServerResult<Markup> {
    Ok(html! {
        [err_component2?]
    })
}

#[page]
pub async fn processing_of_error() -> Markup {
    html! {
        [process_error(err_component2().await)]
    }
}
async fn process_error(component_result: ServerResult<Markup>) -> Markup {
    component_result.unwrap_or_else(|_| html!("An error occurred!"))
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

#[tokio::test]
async fn test_meta_layer_with_page() {
    use {
        crate::{shared::wini::layer::MetaLayerBuilder, template},
        axum::{Router, middleware::from_fn, routing::get},
        axum_test::TestServer,
        std::collections::HashMap,
    };

    let app = Router::new()
        .route("/meta", get(test_meta))
        .route("/default", get(render))
        .layer(
            MetaLayerBuilder::default()
                .default_meta(HashMap::from_iter([
                    ("hello", "world".into()),
                    ("world", "hello".into()),
                    ("keywords", "hello, world".into()),
                ]))
                .force_meta(HashMap::from_iter([("title", "hi world!".into())]))
                .build()
                .unwrap(),
        )
        .layer(from_fn(template::template));
    let server = TestServer::new(app).expect("creates a server");

    let resp = server.get("/meta").await;

    resp.assert_status_ok();
    let txt = resp.text();
    // Title
    assert!(txt.contains("<title>hi world!</title>"));
    assert!(txt.contains(r#"<meta property="og:title" content="hi world!">"#));
    // Keywords
    assert!(txt.contains(r#"<meta name="keywords" content="hello, world">"#));
    // Other meta
    assert!(txt.contains(r#"<meta name="world" content="hello">"#));
    assert!(txt.contains(r#"<meta name="hello" content="world">"#));

    let resp = server.get("/default").await;

    resp.assert_status_ok();
    let txt = resp.text();
    // Title
    assert!(txt.contains("<title>hi world!</title>"));
    assert!(txt.contains(r#"<meta property="og:title" content="hi world!">"#));
    // Keywords
    assert!(txt.contains(r#"<meta name="keywords" content="hello, world">"#));
    // Other meta
    assert!(txt.contains(r#"<meta name="world" content="hello">"#));
    assert!(txt.contains(r#"<meta name="hello" content="world">"#));
}
// ENDIF
