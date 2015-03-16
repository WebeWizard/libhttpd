#![allow(non_snake_case)]
#![allow(unused_parens)]
#![feature(core)]
#![feature(collections)]
#![feature(io)]
#![feature(os)]
#![feature(path_ext)]
#![feature(net)]

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
