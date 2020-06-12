use std::fmt;
use std::io;

pub enum Error {
    IoError(io::Error),
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::IoError(ref e) => e.fmt(formatter),
            &Error::NoDirectory => formatter.write_str("Directory Not Found."),
            &Error::InvalidCommit => formatter.write_str("The commit is not valid."),
            &Error::InvalidIndex => formatter.write_str("The index is corrupt."),
            
        }
    }
}

// Convert io::Error to custom IoError
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}