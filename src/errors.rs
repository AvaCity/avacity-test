use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct WrongChecksum();

impl fmt::Display for WrongChecksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong checksum in message")
    }
}

impl Error for WrongChecksum {}
