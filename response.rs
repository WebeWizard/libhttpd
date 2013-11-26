use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;

use request::{Request, Headers, Header};
use status::Status;

pub struct Response
{
	priv bufStream: BufferedStream<TcpStream>,
	priv status: Status,
	priv headers: Headers,
	priv message: ~[u8]
}

impl Response
{
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
		let status = Status::from_str(~"200");
		return status;
	}
	
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
