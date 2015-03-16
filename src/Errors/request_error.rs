use std::error::Error;
use std::error::FromError;
use std::io;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum RequestError {
	Io(io::Error)
}

impl Error for RequestError {
	fn description(&self) -> &str {
		match *self {
			RequestError::Io(_) => "Encountered IoError"
		}
	}

	fn cause(&self) -> Option<&Error> {
		match *self {
			RequestError::Io(ref err) => Some(err as &Error)
		}
	}
}

impl Display for RequestError {
	fn fmt( &self, f: &mut Formatter ) -> Result {
		write!(f, "Could not build request from stream")
	}
}


impl FromError<io::Error> for RequestError {
	fn from_error(err: io::Error) -> RequestError {
		RequestError::Io(err)
	}
}
