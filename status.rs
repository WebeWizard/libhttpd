pub struct Status
{
	pub code: u16,
	pub reason: String
}

impl Status
{	
	// this is only for standard status codes.
	// if you want a weird status, you'll have to make it yourself.
	pub fn from_code( code: u16 ) -> Status
	{
		let newReason = Status::get_reason( code );
		return Status{ code: code , reason: newReason };
	}

	pub fn get_reason( code: u16 ) -> String
	{
		match code
		{
			//These are all status codes defined by HTTP 1.1
			//This SHOULD NOT be used as a strict list of codes because custom codes are allowed
			100 => "Continue".to_string(),
			101 => "Switching Protocols".to_string(),
			200 => "OK".to_string(),
			201 => "Created".to_string(),
			202 => "Accepted".to_string(),
			203 => "Non-Authoritative Information".to_string(),
			204 => "No Content".to_string(),
			205 => "Reset Content".to_string(),
			206 => "Partial Content".to_string(),
			300 => "Multiple Choices".to_string(),
			301 => "Moved Permanently".to_string(),
			302 => "Found".to_string(),
			303 => "See Other".to_string(),
			304 => "Not Modified".to_string(),
			305 => "Use Proxy".to_string(),
			307 => "Temporary Redirect".to_string(),
			400 => "Bad Request".to_string(),
			401 => "Unauthorized".to_string(),
			402 => "Payment Required".to_string(),
			403 => "Forbidden".to_string(),
			404 => "Not Found".to_string(),
			405 => "Method Not Allowed".to_string(),
			406 => "Not Acceptable".to_string(),
			407 => "Proxy Authentication Required".to_string(),
			408 => "Request Time-out".to_string(),
			409 => "Conflict".to_string(),
			410 => "Gone".to_string(),
			411 => "Length Required".to_string(),
			412 => "Precondition Failed".to_string(),
			413 => "Request Entity Too Large".to_string(),
			414 => "Request-URI Too Large".to_string(),
			415 => "Unsupported Media Type".to_string(),
			416 => "Requested range not satisfiable".to_string(),
			417 => "Expectation Failed".to_string(),
			500 => "Internal Server Error".to_string(),
			501 => "Not Implemented".to_string(),
			502 => "Bad Gateway".to_string(),
			503 => "Service Unavailable".to_string(),
			504 => "Gateway Time-out".to_string(),
			505 => "HTTP Version not supported".to_string(),
			_ => fail!( format!("Unrecognized Status Code: {}", code) ) //TODO: Be able to use custom status codes
		}
	}
}
