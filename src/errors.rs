use std::error::Error as StdError;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct LogLevelParseError {
    pub input: String,
}

impl fmt::Display for LogLevelParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unable to parse log level from input value: {}",
            self.description()
        )
    }
}

impl StdError for LogLevelParseError {
    fn description(&self) -> &str {
        "Unable to parse log level from input value"
    }

    fn cause(&self) -> Option<&StdError> {
        None // there is no causing error
    }
}

#[derive(Debug, Clone)]
pub struct BunyanLogParseError {
    msg: String,
}

impl BunyanLogParseError {
    pub fn new<S>(msg: S) -> BunyanLogParseError
    where
        S: Into<String>,
    {
        BunyanLogParseError { msg: msg.into() }
    }
}

impl fmt::Display for BunyanLogParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl StdError for BunyanLogParseError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }

    fn cause(&self) -> Option<&StdError> {
        None // there is no causing error
    }
}

#[derive(Debug, Clone)]
pub enum ParseIntFromJsonError {
    Structural(BunyanLogParseError),
    Numeric(ParseIntError),
}

impl fmt::Display for ParseIntFromJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseIntFromJsonError::Structural(ref e) => e.fmt(f),
            ParseIntFromJsonError::Numeric(ref e) => e.fmt(f),
        }
    }
}

impl StdError for ParseIntFromJsonError {
    fn description(&self) -> &str {
        match *self {
            ParseIntFromJsonError::Structural(ref e) => e.description(),
            // This already impls `Error`, so defer to its own implementation.
            ParseIntFromJsonError::Numeric(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            ParseIntFromJsonError::Structural(ref e) => Some(e),
            ParseIntFromJsonError::Numeric(ref e) => Some(e),
        }
    }
}

pub type ParseResult = std::result::Result<(), BunyanLogParseError>;

pub struct Error {
    inner: Box<Inner>,
}

struct Inner {
    kind: Kind,
    line: String,
    line_no: usize,
    column: Option<usize>,
}

impl Error {
    pub fn new(kind: Kind, line: String, line_no: usize, column: Option<usize>) -> Error {
        Error {
            inner: Box::new(Inner {
                kind,
                line,
                line_no,
                column,
            }),
        }
    }

    #[inline]
    pub fn line(&self) -> &String {
        &self.inner.line
    }

    #[inline]
    pub fn line_no(&self) -> usize {
        self.inner.line_no
    }

    pub fn column(&self) -> Option<usize> {
        self.inner.column
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.inner.kind)
            .field("line", &self.inner.line)
            .field("line_no", &self.inner.line_no)
            .field("column", &self.inner.column)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner.kind {
            Kind::Json(ref e) => fmt::Display::fmt(e, f),
            Kind::BunyanLogParse(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::Json(ref e) => e.description(),
            Kind::BunyanLogParse(ref e) => e.description(),
        }
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&StdError> {
        match self.inner.kind {
            Kind::Json(ref e) => e.cause(),
            Kind::BunyanLogParse(ref e) => e.cause(),
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    BunyanLogParse(BunyanLogParseError),
    Json(::serde_json::Error),
}

impl From<BunyanLogParseError> for Kind {
    #[inline]
    fn from(error: BunyanLogParseError) -> Kind {
        Kind::BunyanLogParse(error)
    }
}

impl From<::serde_json::Error> for Kind {
    #[inline]
    fn from(error: ::serde_json::Error) -> Kind {
        Kind::Json(error)
    }
}
