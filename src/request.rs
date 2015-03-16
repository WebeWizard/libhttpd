use std::collections::HashMap;
use std::io::{BufStream,BufRead,Read};
use std::net::TcpStream;
use std::io::Error;

pub struct Request {
	pub method: String,
	pub uri: String,
	pub headers: HashMap< String, String >,
	pub messagebody: Box<Read>
}

impl Request {
	pub fn new( bufStream: &mut BufStream<TcpStream> ) -> Result<Request,Error> {
	
		//read in the first line.  Split it into Method, URI, and Query
		let mut line = String::new();
		try!(bufStream.read_line( &mut line ));
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
			let mut header = String::new();
			try!(bufStream.read_line( &mut header ));
			let mut headerSlice = header.as_slice();
			// a \r\n by itself signals the end of the headers
			if ( headerSlice == "\r\n" ) { break; }
			// if it's an actual header, remove the new line chars
			headerSlice = &headerSlice[..headerSlice.len()-2];
			// get header key and value
			let mut headerIter = headerSlice.split( ": " );
			// headers are supposed to be case insensitive, so let's set them all to lowercase
			let headerKey = headerIter.next().unwrap_or("").to_lowercase();
			let headerValue = headerIter.next().unwrap_or("").to_lowercase();
			// insert into headers map
			headers.insert( headerKey, headerValue );
		}
		let mut messageBody = Box::new( "".as_bytes() ) as Box<Read>;
		if ( headers.contains_key(&"content-length".to_string()) || headers.contains_key(&"transfer-encoding".to_string()) ) {
			let newBufStream = BufStream::new( try!( bufStream.get_mut().try_clone() ) );
			messageBody = Box::new( newBufStream );
		}
		
		return Ok( Request{ 
			method: method,
			uri: uri,
			headers: headers,
			messagebody: messageBody
		} );
	}
}
