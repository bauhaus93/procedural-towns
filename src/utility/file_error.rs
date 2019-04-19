use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum FileError {
    IO(io::Error)
}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> FileError {
        FileError::IO(err)
    }
}

impl Error for FileError {

    fn description(&self) -> &str {
        match *self {
            FileError::IO(_) => "io"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            FileError::IO(ref err) => Some(err)
        }
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileError::IO(ref err) => write!(f, "{}:{}", self.description(), err)
        }
    }
}
