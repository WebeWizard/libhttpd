use std::io::fs;
use std::hashmap::HashMap;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use extra::time;
use extra::time::Tm;

pub struct Headers;

impl Headers
{
	//General Headers - http://www.w3.org/Protocols/rfc2616/rfc2616-sec4.html#sec4.5
	pub fn getDateHeader() -> ~str
	{
		let time: Tm = time::now_utc();
		return format!( "Date: {}\r\n", time.rfc822() ); //time in the format:  Sun, 06 Nov 1994 08:49:37 GMT
	}

	//Request Headers - http://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html#sec5.3

	//Response Headers - http://www.w3.org/Protocols/rfc2616/rfc2616-sec6.html#sec6.2
	pub fn getContentLengthHeader( path: &Path ) -> ~str
	{
		let size = fs::stat( path ).size;
		return format!( "Content-Length: {}\r\n", size) ; 
	}
	
	pub fn writeToStream( headers: &HashMap<~str,~str> , bufStream: &mut BufferedStream<TcpStream> )
	{
		for ( key, value ) in headers.iter()
		{
			let headerStr = format!( "{}: {}\r\n" , key.to_owned(), value.to_owned() );
			bufStream.write( headerStr.as_bytes() );
		}
	}
}


