use core::fmt;
use std::{error, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    SerializationError(rblitz_packets::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Debug).fmt(fmt)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<rblitz_packets::Error> for Error {
    fn from(e: rblitz_packets::Error) -> Self {
        Error::SerializationError(e)
    }
}
