use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;

use method::Method;
use requesturi::RequestURI;

pub struct Request
{
	priv bufStream: BufferedStream<TcpStream>,
	priv method: Method,
	priv uri: RequestURI,
	priv headers: Headers
}

pub struct Headers
{
	priv headers: ~[Header]
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
		let mut requestIter = requestLine.word_iter();
		
		let method = Method::from_str( requestIter.next().unwrap() );
		let uri = RequestURI::from_str( &method, requestIter.next().unwrap() );
		
		//read all remaining lines of the header
		let mut headersVector: ~[Header] = ~[];
		loop
		{
			let line = bufStream.read_line().unwrap();
			if (line == "\r\n".to_str()) { break; }

			let mut lineIter = line.split_iter(' ');
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
	
	pub fn respond(&mut self)
	{
		self.bufStream.write(bytes!("I am the greatest\r\n"));
		self.bufStream.flush();
	}

}
