pub struct Status {
	pub code: u16,
	pub reason: &'static str
}

impl Status {
	// this is only for standard status codes.
	// if you want a custom status, you'll have to make it yourself
	pub fn from_code( code: u16 ) -> Option<Status> {
		match Status::get_reason(code) {
			Some(reason) => Some(Status{ code: code , reason: reason }),
			None => None
		}
	}
	
	// these are all status codes defined by HTTP 1.1
	// this SHOULD NOT be used as a strict list of codes because custom codes are allowed
	pub fn get_reason( code: u16 ) -> Option<&'static str> {
		match code {
			100 => Some("Continue"),
			101 => Some("Switching Protocols"),
			200 => Some("OK"),
			201 => Some("Created"),
			202 => Some("Accepted"),
			203 => Some("Non-Authoritative Information"),
			204 => Some("No Content"),
			205 => Some("Reset Content"),
			206 => Some("Partial Content"),
			300 => Some("Multiple Choices"),
			301 => Some("Moved Permanently"),
			302 => Some("Found"),
			303 => Some("See Other"),
			304 => Some("Not Modified"),
			305 => Some("Use Proxy"),
			307 => Some("Temporary Redirect"),
			400 => Some("Bad Request"),
			401 => Some("Unauthorized"),
			402 => Some("Payment Required"),
			403 => Some("Forbidden"),
			404 => Some("Not Found"),
			405 => Some("Method Not Allowed"),
			406 => Some("Not Acceptable"),
			407 => Some("Proxy Authentication Required"),
			408 => Some("Request Time-out"),
			409 => Some("Conflict"),
			410 => Some("Gone"),
			411 => Some("Length Required"),
			412 => Some("Precondition Failed"),
			413 => Some("Request Entity Too Large"),
			414 => Some("Request-URI Too Large"),
			415 => Some("Unsupported Media Type"),
			416 => Some("Requested range not satisfiable"),
			417 => Some("Expectation Failed"),
			500 => Some("Internal Server Error"),
			501 => Some("Not Implemented"),
			502 => Some("Bad Gateway"),
			503 => Some("Service Unavailable"),
			504 => Some("Gateway Time-out"),
			505 => Some("HTTP Version not supported"),
			_ => None
		}
	}
}
