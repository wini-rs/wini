use {
    crate::shared::wini::err::{ServerErrorKind, ServerResult},
    axum::body::Bytes,
    http_body_util::BodyExt,
    std::fmt::Debug,
};


/// Converts an axum body to String
pub async fn buffer_to_string<B>(body: B) -> ServerResult<String>
where
    B: axum::body::HttpBody<Data = Bytes> + Debug,
    B::Error: std::fmt::Display + std::fmt::Debug,
{
    Ok(std::str::from_utf8(
        &body
            .collect()
            .await
            .map_err(|e| ServerErrorKind::DebugedError(format!("{e}")))?
            .to_bytes(),
    )?
    .to_string())
}
