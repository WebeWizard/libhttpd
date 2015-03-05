use std::collections::HashMap;
use std::old_io::BufferedStream;
use std::old_io::net::tcp::TcpStream;

pub fn write_to_stream( headers: &HashMap<String,String> , bufStream: &mut BufferedStream<TcpStream> )
{
	for ( key, value ) in headers.iter()
	{
		let headerStr = format!( "{}: {}\r\n" , key.to_string(), value.to_string() );
		let result = bufStream.write_all( headerStr.as_bytes() );
		match result {
			Ok(()) => {},
			Err(error) => { println!("Header write error: {}", error); }
		}
	}
}
