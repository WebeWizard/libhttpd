use std::collections::hashmap::HashMap;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use method::Method;
use request::Request;

pub struct Context
{
	pub methods: Vec<Method>,
	pub subContextMap: HashMap< String , Context >,
	pub action:	fn( &Request , &mut BufferedStream<TcpStream> ) -> ()
}

impl Clone for Context
{
	fn clone( &self ) -> Context
	{
		let methods = self.methods.clone();
		let subContextMap = self.subContextMap.clone();
		let action = self.action;
	
		return Context{ methods: methods, subContextMap: subContextMap, action: action};
	}
}
