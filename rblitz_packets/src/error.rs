use std::fmt;

use serde::{de, ser};
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Message(String),
    TooMuchData(usize, usize),
    UnexpectedEof,
    Utf8Error(std::str::Utf8Error),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Message(ref msg) => fmt.write_str(msg),
            Error::TooMuchData(received, max) => write!(
                fmt,
                "received {} values to serialize with a possible max of {}",
                received, max
            ),
            Error::UnexpectedEof => fmt.write_str("unexpected end of data"),
            Error::Utf8Error(e) => e.fmt(fmt),
        }
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::UnexpectedEof
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::Utf8Error(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error(e.utf8_error())
    }
}

impl std::error::Error for Error {}
