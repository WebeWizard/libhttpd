use std::error::Error;
use std::error::FromError;
use std::old_io::IoError;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum RequestError {
	Io(IoError)
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


impl FromError<IoError> for RequestError {
	fn from_error(err: IoError) -> RequestError {
		RequestError::Io(err)
	}
}
