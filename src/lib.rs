#![allow(non_snake_case)]
#![allow(unnecessary_parens)]
#![allow(unused_parens)]

extern crate flate2;

pub mod server;
pub mod method;
pub mod encoder;
pub mod request;
pub mod response;
pub mod status;
pub mod headers;

pub mod Methods
{
	pub mod GET;
}

pub mod Encoders
{
	pub mod chunked;
	pub mod deflate;
	pub mod gzip;
}

pub mod Errors
{
	pub mod stream_error;
	pub mod request_error;
}
