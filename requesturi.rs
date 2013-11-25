use extra::url;

use method::Method;

pub enum RequestURI
{
	AbsolutePath(~str),
	AbsoluteURI(url::Url),
	Authority(~str),
	Wildcard
}

impl RequestURI
{
	pub fn from_str(method: &Method, s: &str) -> RequestURI
	{
		match method
		{
			//CONNECT: for use with a proxy that can dynamically switch to being a tunnel (ssh).
			//CONNECT: Only possible URI is an Authority( hostname, ip, network location, etc)
			//CONNECT => { Authority(s.to_str()) },
			
			//Anything else
			_ =>
			{
				let length = s.len();
				if (length == 1)
				{
					match s.char_at(0)
					{
						'*' => { Wildcard },
						'/' => { AbsolutePath(~"/") },
						_ => { fail!(format!("Invalid URI: {:s}", s)); }
					}
				}
				else
				{
					match s.char_at(0)
					{
						'/' => { AbsolutePath(s.to_str()) },
						_ => 
						{
							match url::from_str(s)
							{
								Ok(url) => AbsoluteURI(url),
								Err(err) => fail!(err)
							}
						}
					}
				}
			}
		}
	}
}
