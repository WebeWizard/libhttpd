use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;

use method::Method;

pub struct Request
{
	bufStream: BufferedStream<TcpStream>,
	method: Method,
	uri: ~str,
	headers: Headers
}

pub struct Headers
{
	headers: ~[Header]
}

pub struct Header
{
	priv key: ~str,
	priv value: ~str
}

impl Request
{
	pub fn new(tcpStream: TcpStream) -> Request
	{
		//wrap the stream in a buffer
		let mut bufStream = BufferedStream::new(tcpStream);
		
		//create an iterator to split request line into words (separated by any white space)
		let requestLine = bufStream.read_line().unwrap();
		let mut requestIter = requestLine.words();
		
		let method = Method::from_str( requestIter.next().unwrap() );
		let uri = requestIter.next().unwrap().to_owned();
		
		//read all remaining lines of the header
		let mut headersVector: ~[Header] = ~[];
		loop
		{
			let line = bufStream.read_line().unwrap();
			if (line == "\r\n".to_str()) { break; } //a blank (\r\n) line means the end of the request

			let mut lineIter = line.split(' ');
			let key = lineIter.next().unwrap().to_str();
			let value = lineIter.next().unwrap().to_str();
			headersVector.push( Header { key: key, value: value } );
		}
		let headers = Headers { headers: headersVector };
		
		//build the request from gathered parts
		let request = Request { 
			bufStream: bufStream,
			method: method,
			uri: uri,
			headers: headers
		};
		
		return request;
	}
}
