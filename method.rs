pub enum Method
{
	CONNECT,
	DELETE,
	GET,
	HEAD,
	OPTIONS,
	POST,
	PUT,
	TRACE
}

impl Method
{
	pub fn from_str(s: &str) -> Method
	{
		match s
		{
			"CONNECT" => CONNECT,
			"DELETE" => DELETE,
			"GET" => GET,
			"HEAD" => HEAD,
			"OPTIONS" => OPTIONS,
			"POST" => POST,
			"PUT" => PUT,
			"TRACE" => TRACE,
			//catch unrecognized methods and fail
			_ => { fail!(format!("Unrecognized method: {:s}", s)); }
		}
	}
}

