use {
    crate::shared::wini::{
        cache::{AddCache, CacheCategory},
        config::SERVER_CONFIG,
        err::{ServerErrorKind, ServerResult},
        CSS_FILES,
        JS_FILES,
        PUBLIC_ENDPOINTS,
    },
    axum::{
        extract::Request,
        http::HeaderValue,
        response::{IntoResponse, Response},
    },
    hyper::{header::CONTENT_TYPE, StatusCode},
    tower_http::services::ServeFile,
};

/// This request handler is in charge of serving files.
/// In particular, these kind of files, in that order:
/// - public files (exposed files)
/// - css files
/// - javascript files
pub async fn handle_file(req: Request) -> ServerResult<Response<axum::body::Body>> {
    let path = &req.uri().path().to_string();

    if PUBLIC_ENDPOINTS.contains(path) {
        return Ok(ServeFile::new(format!("./public{path}"))
            .try_call(req)
            .await
            .map_err(|_| ServerErrorKind::PublicRessourceNotFound(path.clone()))?
            .into_response());
    }

    if path.ends_with(".css") {
        return if let Some(file) = CSS_FILES.get(path) {
            css_into_response(file)
        } else {
            Err(StatusCode::NOT_FOUND.into())
        };
    }

    if path.ends_with(".js") {
        return if let Some(file) = JS_FILES.get(path) {
            js_into_response(file)
        } else {
            Err(StatusCode::NOT_FOUND.into())
        };
    }

    Err(StatusCode::NOT_FOUND.into())
}

fn js_into_response(file_content: &str) -> ServerResult<Response<axum::body::Body>> {
    file_into_response(file_content, "javascript")?
        .add_cache(&SERVER_CONFIG.cache.get_or_panic(CacheCategory::Javascript))
}

fn css_into_response(file_content: &str) -> ServerResult<Response<axum::body::Body>> {
    file_into_response(file_content, "css")?
        .add_cache(&SERVER_CONFIG.cache.get_or_panic(CacheCategory::Css))
}

/// Create a response from the content of the file and add the content_type header accordingly with
/// the kind of content_type passed in parameter of this function.
fn file_into_response(file_content: &str, kind: &str) -> ServerResult<Response<axum::body::Body>> {
    Ok((
        [(
            CONTENT_TYPE,
            HeaderValue::from_str(&format!("text/{kind}; charset=utf-8"))?,
        )],
        Box::<str>::from(file_content),
    )
        .into_response())
}
