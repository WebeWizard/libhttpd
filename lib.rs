#![crate_id = "httpd#0.2"]
#![crate_type = "lib"]

#![allow(unnecessary_parens)]

extern crate time;

pub mod server;
pub mod method;
pub mod request;
pub mod response;
pub mod status;
pub mod headers;
pub mod encoder;

pub mod methods
{
	pub mod GET;
}

pub mod encoders
{
	pub mod identity;
	pub mod chunked;
}
