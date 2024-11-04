use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    while_parsing: &'static str,
}

impl ParseError {
    pub(crate) fn at_identifier() -> Self {
        ParseError {
            while_parsing: "AT-identifier",
        }
    }

    pub(crate) fn at_uri() -> Self {
        ParseError {
            while_parsing: "AT-URI",
        }
    }

    pub(crate) fn datetime() -> Self {
        ParseError {
            while_parsing: "DateTime",
        }
    }

    pub(crate) fn did() -> Self {
        ParseError {
            while_parsing: "DID",
        }
    }

    pub(crate) fn did_url() -> Self {
        ParseError {
            while_parsing: "DID URL",
        }
    }

    pub(crate) fn handle() -> Self {
        ParseError {
            while_parsing: "Handle",
        }
    }

    pub(crate) fn nsid() -> Self {
        ParseError {
            while_parsing: "NSID",
        }
    }

    pub(crate) fn nsid_fragment() -> Self {
        ParseError {
            while_parsing: "NSID fragment",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error parsing {}", self.while_parsing)
    }
}

impl std::error::Error for ParseError {}
