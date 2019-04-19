use std::fmt;
use std::error::Error;

use super::file_error::FileError;

#[derive(Debug)]
pub enum ApplicationError {
    File(FileError)
}

impl From<FileError> for ApplicationError {
    fn from(err: FileError) -> Self {
        ApplicationError::File(err)
    }
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::File(_) => "file"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::File(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::File(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}
