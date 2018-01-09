use std::io;
use std::io::ErrorKind::*;

pub fn nonzero_error() -> io::Error {
    io::Error::new(InvalidInput, "Need nonzero document length")
}

pub fn invalid(err: &str) -> io::Error {
    io::Error::new(InvalidData, err)
}
