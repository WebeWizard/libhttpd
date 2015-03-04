use std::error::Error;
use std::error::FromError;
use std::old_io::IoError;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use Errors::request_error::RequestError;

pub enum StreamError {
	Request(RequestError),
	Io(IoError)
}

impl Error for StreamError {
	fn description(&self) -> &str {
		match *self {
			StreamError::Request(_) => "Encountered RequestError",
			StreamError::Io(_) => "Encountered IoError"
		}
	}

	fn cause(&self) -> Option<&Error> {
		match *self {
			StreamError::Request(ref err) => Some(err as &Error ),
			StreamError::Io(ref err) => Some(err as &Error)
		}
	}
}

impl Display for StreamError {
	fn fmt( &self, f: &mut Formatter ) -> Result {
		write!(f, "Encountered an Error with the Stream")
	}
}


impl FromError<RequestError> for StreamError {
	fn from_error(err: RequestError) -> StreamError {
		StreamError::Request(err)
	}
}

impl FromError<IoError> for StreamError {
	fn from_error(err: IoError) -> StreamError {
		StreamError::Io(err)
	}
}
