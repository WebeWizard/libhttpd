use std::collections::HashMap;
use std::old_io::BufferedStream;
use std::old_io::net::tcp::TcpStream;
use std::old_io::IoResult;

pub struct Request {
	pub method: String,
	pub uri: String,
	pub headers: HashMap< String, String >,
	pub messagebody: String
}

impl Request {
	pub fn new( bufStream: &mut BufferedStream<TcpStream> ) -> IoResult<Request> {
	
		//read in the first line.  Split it into Method, URI, and Query
		let line = try!(bufStream.read_line());
		let lineSlice = line.as_slice();
		let mut iter = lineSlice.split( ' ' );
		// get Method
		let method = iter.next().unwrap().to_string();
		// get URI
		let uri = iter.next().unwrap().to_string();
		// get Headers
		let mut headers = HashMap::<String,String>::new();
		loop
		{
			let header = try!(bufStream.read_line());
			let mut headerSlice = header.as_slice();
			// a \r\n by itself signals the end of the headers
			if ( headerSlice == "\r\n" ) { break; }
			// if it's an actual header, remove the new line chars
			headerSlice = &headerSlice[..headerSlice.len()-2];
			//get header key and value
			let mut headerIter = headerSlice.split( ": " );
			let headerKey = headerIter.next().unwrap_or("").to_string();
			let headerValue = headerIter.next().unwrap_or("").to_string();
			//insert into headers map
			headers.insert( headerKey, headerValue );
		}
		
		let mut messagebody = String::new();
		if ( headers.contains_key(&"Content-Length".to_string()) || headers.contains_key(&"Transfer-Encoding".to_string()) ) {
			messagebody = try!( bufStream.read_to_string() );
		}
		
		return Ok( Request{ 
			method: method,
			uri: uri,
			headers: headers,
			messagebody: messagebody
		} );
	}
}
