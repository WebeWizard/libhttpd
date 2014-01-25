use std::hashmap::HashMap;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use method::Method;
use request::Request;

pub struct Context
{
	name: ~str,
	methods: ~[Method],
	subContextMap: HashMap<~str, Context>,
	action:	fn( &Request , &mut BufferedStream<TcpStream> ) -> ()
}
