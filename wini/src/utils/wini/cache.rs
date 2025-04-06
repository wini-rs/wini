use {
    crate::shared::wini::{cache::CacheCategory, config::SERVER_CONFIG, err::ServerResult},
    axum::{http::HeaderValue, middleware::Next, response::Response},
    hyper::header::CACHE_CONTROL,
};

/// Add cache to an axum response
pub fn add_cache(mut response: Response, cache_rule: &str) -> ServerResult<Response> {
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_str(cache_rule)?);

    Ok(response)
}

/// Add the HTML cache rule
pub async fn html_middleware(
    req: hyper::Request<axum::body::Body>,
    next: Next,
) -> ServerResult<Response> {
    let rep = next.run(req).await;
    let (mut res_parts, res_body) = rep.into_parts();
    res_parts.headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str(&SERVER_CONFIG.cache.get_or_panic(CacheCategory::Html))?,
    );
    let res = Response::from_parts(res_parts, res_body);
    Ok(res)
}
