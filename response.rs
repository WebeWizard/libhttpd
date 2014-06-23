use std::collections::hashmap::HashMap;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use headers;
use request::Request;
use method::{ CONNECT,DELETE,GET,HEAD,OPTIONS,POST,PUT,TRACE };
use methods::GET;
use status::Status;

use encoder::Encoder;

pub enum ResponseType
{
	FILE,
	DIR,
	ERROR
}

pub struct Response {
	pub status: Status,
	pub responseType: ResponseType,
	pub encoder: Encoder,
	pub headers:  HashMap< String, String >,
	pub messageSender: fn(  &Request, &Response, &mut BufferedStream<TcpStream> ) -> bool
}

impl Response
{
	pub fn new( request: &Request ) -> Option<Response>
	{
		match request.method
		{
			// ALL METHODS MUST RETURN Option<Response>
			CONNECT =>
			{
			    //NOT IMPLEMENTED YET
			    return None;
			},
			DELETE =>
			{
			    //NOT IMPLEMENTED YET
			    return None;
			},
			GET =>
			{
				return Some(GET::response( request ));
			},
			HEAD =>
			{
			    //NOT IMPLEMENTED YET
			    return None;
			},
			OPTIONS =>
			{
				//NOT IMPLEMENTED YET
				return None;
			},
			POST =>
			{
			    //NOT IMPLEMENTED YET
			    return None;
			},
			PUT =>
			{
			    //NOT IMPLEMENTED YET
			    return None;
			},
			TRACE =>
			{
			    //NOT IMPLEMENTED YET
			    return None;
			},
		}
	}
	
	pub fn respond( &mut self, request: &Request, bufStream: &mut BufferedStream<TcpStream> ) -> bool
	{
		let mut successFlag = true;
		
		// write status line
		let statusLine = format!( "HTTP/1.1 {} {}\r\n", self.status.code, self.status.reason );
		bufStream.write( statusLine.as_bytes() );
		
		// if we allow keep-alive, then use it
		self.headers.insert( "Connection".to_string(), "keep-alive".to_string() );
		
		// write headers
		headers::write_to_stream( &self.headers , bufStream);
		
		// end headers with an empty line
		bufStream.write( "\r\n".as_bytes() );
		
		// write the message
		successFlag = (self.messageSender)( request, self, bufStream );
		
		// flush the stream
		bufStream.flush();
		
		return successFlag;
	}
}	
