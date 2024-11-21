use std::fmt;

use tokio_tungstenite::tungstenite;

/// Error type for Jetstream operations.
#[derive(Debug)]
pub enum Error {
    /// An HTTP protocol error.
    Http(http::Error),
    /// An error produced during JSON serialization or deserialization.
    Json(serde_json::Error),
    /// A WebSocket protocol error.
    WebSocket(tungstenite::Error),
}

impl From<http::Error> for Error {
    #[inline]
    fn from(e: http::Error) -> Self {
        Error::Http(e)
    }
}

impl From<serde_json::Error> for Error {
    #[inline]
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<tungstenite::Error> for Error {
    #[inline]
    fn from(e: tungstenite::Error) -> Self {
        Error::WebSocket(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => fmt::Display::fmt(e, f),
            Error::Json(e) => fmt::Display::fmt(e, f),
            Error::WebSocket(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::WebSocket(e) => Some(e),
        }
    }
}
