use std::fmt;

use tokio_tungstenite::tungstenite::{self, protocol::CloseFrame};

/// Error type for Jetstream operations.
#[derive(Debug)]
pub enum Error {
    /// The underlying WebSocket was closed by the remote peer.
    Closed(Option<CloseFrame<'static>>),
    /// An HTTP protocol error occurred.
    Http(http::Error),
    /// An error occurred during JSON serialization or deserialization.
    Json(serde_json::Error),
    /// A WebSocket protocol error occurred.
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
            Error::Closed(opt) => match opt {
                Some(c) => write!(f, "WebSocket closed by remote peer: {}", &c.reason),
                None => f.write_str("WebSocket closed by remote peer"),
            },
            Error::Http(e) => fmt::Display::fmt(e, f),
            Error::Json(e) => fmt::Display::fmt(e, f),
            Error::WebSocket(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Closed(_) => None,
            Error::Http(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::WebSocket(e) => Some(e),
        }
    }
}
