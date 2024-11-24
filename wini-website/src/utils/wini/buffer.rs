use {
    crate::shared::wini::err::ServerResult,
    axum::body::Bytes,
    http_body_util::BodyExt,
    std::fmt::Debug,
};


/// Converts an axum body to String
pub async fn buffer_to_string<B>(body: B) -> ServerResult<String>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B: Debug,
    B::Error: std::fmt::Display + std::fmt::Debug,
{
    let bytes = body.collect().await.unwrap().to_bytes();
    Ok(std::str::from_utf8(&bytes).unwrap().to_string())
}
