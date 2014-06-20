use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;
use std::collections::hashmap::HashMap;

use method::Method;

pub struct Request
{
	pub method: Method,
	pub uri: String,
	pub query: String,
	pub headers: HashMap< String, String >
}

impl Request
{
	pub fn new(bufStream: &mut BufferedStream<TcpStream>) -> Option<Request>
	{
		//read in the first line.  Split it into Method, URI, and Query
		let lineTest = bufStream.read_line();
		if ( lineTest.is_err() ) { return None; }
		let line = lineTest.unwrap();
		let lineSlice = line.as_slice();
		// --- get method
		let mut requestStringIter = lineSlice.split( ' ' );
		let method = requestStringIter.next().unwrap();
		// --- get uri and query
		let uriQuery = requestStringIter.next().unwrap();
		let mut uriQueryIter = uriQuery.split( '?' );
		let uri = uriQueryIter.next().unwrap_or("").into_string();
		let query = uriQueryIter.next().unwrap_or("").into_string();
	
		//read the rest of the headers
		let mut headers = HashMap::<String,String>::new();
		loop
		{
			let header = bufStream.read_line().unwrap();
			let mut headerSlice = header.as_slice();
			// a \r\n by itself signals the end of the headers
			if ( headerSlice == "\r\n" ) { break; }
			// if it's an actual header, remove the new line chars
			headerSlice = headerSlice.slice_to( headerSlice.len()-2 );
			
			//get header key and value
			let mut headerIter = headerSlice.split_str( ": " );
			let headerKey = headerIter.next().unwrap_or("").into_string();
			let headerValue = headerIter.next().unwrap_or("").into_string();
			//insert into headers map
			//println!("key: {}, value: {}",headerKey,headerValue);
			headers.insert( headerKey, headerValue );
			
		}
	
		let newRequest = Request{ 
			method: Method::from_str( method ),
			uri: uri,
			query: query,
			headers: headers
		};
		
		return Some(newRequest);
	}
}
