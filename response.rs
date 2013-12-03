use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;

use request::{Request, Headers, Header};
use status::Status;
use method::{ CONNECT,DELETE,GET,HEAD,OPTIONS,POST,PUT,TRACE };
use methods::GET;

pub struct Response
{
	priv bufStream: BufferedStream<TcpStream>,
	priv status: Status,
	priv headers: Headers,
	priv message: ~[u8]
}

impl Response
{
	//new:	builds a response from the request.
	pub fn new( request: Request ) -> Response
	{
		let status = Response::getStatus( &request );
		
		let mut headersVector: ~[Header] = ~[];
		let headers = Headers { headers: headersVector };
		
		let response = Response { 
			bufStream: request.bufStream,
			status: status,
			headers: headers,
			message: bytes!("poop").to_owned()
		};
		return response;
	}
	
	pub fn getStatus( request: &Request ) -> Status
	{
		let mut status;
		
		match request.method
		{
			CONNECT =>
			{
				//NOT IMPLEMENTED YET
				status = Status::from_str(~"501");
			},
			DELETE =>
			{
				//NOT IMPLEMENTED YET
				status = Status::from_str(~"501");
			},
			GET =>
			{
				let path: &str = request.uri;
				status = GET::validate( path );
			},
			HEAD =>
			{
				//NOT IMPLEMENTED YET
				status = Status::from_str(~"501");
			},
			OPTIONS =>
			{
				//NOT IMPLEMENTED YET
				status = Status::from_str(~"501");
			},
			POST =>
			{
				//NOT IMPLEMENTED YET
				status = Status::from_str(~"501");
			},
			PUT =>
			{
				//NOT IMPLEMENTED YET
				status = Status::from_str(~"501");
			},
			TRACE =>
			{
				//NOT IMPLEMENTED YET
				status = Status::from_str(~"501");
			},
		}
		return status;
	}
	
	//respond:  writes the information stored in the Response struct into the struct's bufStream
	pub fn respond( mut self )
	{
		write!(&mut self.bufStream as &mut Writer, "HTTP/1.1 {:s} {:s}\r\n", self.status.statusCode, self.status.reason);
		self.bufStream.flush();
		self.bufStream.write(bytes!("\r\n"));
		self.bufStream.flush();
		self.bufStream.write(bytes!("poop"));
		self.bufStream.flush();
	}
}
