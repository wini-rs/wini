use {
    axum::response::{IntoResponse, Response},
    hyper::{
        header::{InvalidHeaderValue, ToStrError},
        StatusCode,
    },
    maud::Markup,
    std::{convert::Infallible, str::Utf8Error, sync::Arc},
};

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Clone)]
pub struct ServerError {
    kind: Arc<ServerErrorKind>,
    trace: Option<Vec<Trace>>,
}

impl ServerError {
    pub fn add_trace(&mut self, trace: Trace) {
        match &mut self.trace {
            Some(curr_trace) => {
                curr_trace.push(trace);
            },
            None => {
                self.trace = Some(vec![trace]);
            },
        }
    }
}

#[derive(Debug)]
pub enum ServerErrorKind {
    Status(hyper::StatusCode),
    Infallible(Infallible),
    Utf8Error(Utf8Error),
    InvalidHeader(InvalidHeaderValue),
    DebugedError(String),
    PublicRessourceNotFound(String),
    ToStrError(ToStrError),
}

impl From<ServerErrorKind> for ServerError {
    fn from(kind: ServerErrorKind) -> Self {
        ServerError {
            kind: Arc::new(kind),
            trace: None,
        }
    }
}

/// Macro to easily implement errors into
macro_rules! impl_from_error {
    ($from:ty, $to:path) => {
        impl From<$from> for ServerError {
            fn from(rejection: $from) -> Self {
                ServerError {
                    kind: Arc::new($to(rejection)),
                    trace: None,
                }
            }
        }

        impl From<$from> for ServerErrorKind {
            fn from(rejection: $from) -> Self {
                $to(rejection)
            }
        }
    };
}

impl_from_error!(hyper::StatusCode, ServerErrorKind::Status);
impl_from_error!(Infallible, ServerErrorKind::Infallible);
impl_from_error!(Utf8Error, ServerErrorKind::Utf8Error);
impl_from_error!(String, ServerErrorKind::DebugedError);
impl_from_error!(InvalidHeaderValue, ServerErrorKind::InvalidHeader);
impl_from_error!(ToStrError, ServerErrorKind::ToStrError);


impl IntoResponse for &ServerErrorKind {
    fn into_response(self) -> Response {
        eprintln!("{self:#?}");
        let err_msg = match self {
            ServerErrorKind::InvalidHeader(err) => {
                format!("Unexpected header value: {err}")
            },
            ServerErrorKind::DebugedError(err) => {
                format!("Unexpected error: {err}")
            },
            ServerErrorKind::Infallible(err) => {
                format!("This error should not be possible: {err:#?}")
            },
            ServerErrorKind::ToStrError(err) => {
                format!("Invalid str: {err}")
            },
            ServerErrorKind::Utf8Error(err) => {
                format!("Error decoding buffer to UTF-8: {err:#?}")
            },
            ServerErrorKind::PublicRessourceNotFound(path) => {
                return (StatusCode::NOT_FOUND, format!("Couldn't find file: {path}"))
                    .into_response();
            },
            ServerErrorKind::Status(status_code) => return status_code.into_response(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response()
    }
}

impl IntoResponse for ServerErrorKind {
    fn into_response(self) -> Response {
        (&self).into_response()
    }
}

impl IntoResponse for &ServerError {
    fn into_response(self) -> Response {
        self.kind.as_ref().into_response()
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        self.kind.as_ref().into_response()
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
    fn exit_with_msg_to_compute_if_err<S: std::fmt::Display, F: Fn() -> S>(self, msg: F) -> T;
}

impl<T, E> ExitWithMessageIfErr<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn exit_with_msg_if_err(self, msg: impl std::fmt::Display) -> T {
        self.map_err(|err| {
            log::error!("{msg}: {err:?}");
            std::process::exit(1);
        })
        .expect("Already exited if `Err`")
    }

    fn exit_with_msg_to_compute_if_err<S: std::fmt::Display, F: Fn() -> S>(self, msg: F) -> T {
        self.map_err(|err| {
            log::error!("{}: {err:?}", msg());
            std::process::exit(1);
        })
        .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Backtrace {
    pub markup: Option<Markup>,
    pub err: Arc<ServerErrorKind>,
    // First element is the oldest
    pub trace: Vec<Trace>,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub file_path: &'static str,
    pub function_name: &'static str,
}

impl From<ServerError> for Backtrace {
    fn from(value: ServerError) -> Self {
        Self {
            markup: None,
            err: value.kind,
            trace: value.trace.unwrap_or_default(),
        }
    }
}
