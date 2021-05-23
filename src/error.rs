use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::HTTP(_) => {
                write!(f, "http error")
            }
            ErrorKind::Status(status) => {
                write!(f, "status error {}", status)
            }
            ErrorKind::JSON(_) => {
                write!(f, "json error")
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self {
            kind: ErrorKind::HTTP(e),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self {
            kind: ErrorKind::JSON(e),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
#[derive(Debug)]
pub enum ErrorKind {
    HTTP(reqwest::Error),
    Status(reqwest::StatusCode),
    JSON(serde_json::Error),
}
