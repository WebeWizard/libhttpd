use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;

pub struct Request
{
	priv tcpStream: TcpStream,
	priv method: ~str, //change this to Method Enum later
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
		let headers = Headers::parseHeaders(&tcpStream);
		let request = Request { tcpStream: tcpStream, method: ~"something for now", headers: headers };
		return request;
	}
	
	pub fn respond(&mut self)
	{
		self.tcpStream.write(bytes!("I am the greatest\r\n"));
	}

}

impl Headers
{
	pub fn parseHeaders(tcpStream: &TcpStream) -> Headers
	{
		let headers = Headers { headers: ~[ Header { key: ~"i am a key" , value: ~"i am a value" } ] };
		return headers;
	}	
}
