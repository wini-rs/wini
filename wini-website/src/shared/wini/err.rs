use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum ServerError {
    Status(hyper::StatusCode),
}

/// Macro to implement erros into
macro_rules! impl_from_error {
    ($from:ty, $to:path) => {
        impl From<$from> for ServerError {
            fn from(rejection: $from) -> Self {
                $to(rejection)
            }
        }
    };
}

impl_from_error!(hyper::StatusCode, Self::Status);

pub type ServerResult<T> = Result<T, ServerError>;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        eprintln!("{:#?}", self);
        match self {
            Self::Status(status_code) => status_code.into_response(),
        }
    }
}


/// A trait for handling `Result` types by exiting the program with a custom error message if an error occurs.
///
/// This trait provides a convenient way to handle errors in situations where encountering an error
/// should result in immediate program termination with a meaningful error message.
///
/// # Example
///
/// ```ignore
/// use wini::shared::wini::err::ExitWithMessageIfErr;
///
/// fn main() {
///     // Will exit with error message if file cannot be opened
///     let file = std::fs::File::open("config.txt")
///         .exit_with_msg_if_err("Failed to open configuration file");
///
///     // Continue processing with file...
/// }
/// ```
///
/// # Panics
///
/// This trait implementation never panics. Instead, it exits the program with status code 1
/// when encountering an error.
pub trait ExitWithMessageIfErr<T> {
    /// Handles a `Result` by either returning the success value or exiting the program
    /// with a custom error message if an error occurs.
    fn exit_with_msg_if_err(self, msg: impl std::fmt::Display) -> T;
}

impl<T, E> ExitWithMessageIfErr<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn exit_with_msg_if_err(self, msg: impl std::fmt::Display) -> T {
        self.map_err(|err| {
            // colog::init();
            log::error!("{msg}: {err:?}");
            std::process::exit(1);
        })
        .unwrap()
    }
}
