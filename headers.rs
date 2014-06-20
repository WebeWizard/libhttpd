use std::collections::hashmap::HashMap;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

pub fn write_to_stream( headers: &HashMap<String,String> , bufStream: &mut BufferedStream<TcpStream> )
{
	for ( key, value ) in headers.iter()
	{
		let headerStr = format!( "{}: {}\r\n" , key.to_string(), value.to_string() );
		bufStream.write( headerStr.as_bytes() );
	}
}
