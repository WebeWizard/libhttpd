Example of how to make the server, and add contexts to it

```Rust

extern mod httpd;

use std::io::net::ip::Ipv4Addr;
use std::hashmap::HashMap;

use httpd::server::Server;
use httpd::context::Context;

//this function is the action to be performed when the 'hello' context is used ( http://mysite/hello )
fn hello()
{
	println!("hello from a Context");
}

//this function is the action to be performed when 'world', a subcontext of 'hello', is used ( http://mysite/hello/world )
fn world()
{
	println!("hello from a Sub Context!");
}

fn main()
{
	println("Starting web server...");
	//put it in a new thread so it doesn't block any other logic we may want outside of the web server
	do spawn {
		//create a new server with a defined ip and port
		let mut myserver: Server = Server::newFromIpAddr( Ipv4Addr(127,0,0,1), 9123 );
		
		//create the sub context 'world' struct
		let mySubContext = Context{ name: ~"world", methods: ~[], subContextMap: HashMap::<~str, Context>::new(), action: world};
		//create the main context 'hello' struct
		let mut myContext = Context{ name: ~"hello", methods: ~[], subContextMap: HashMap::<~str, Context>::new(), action: hello};
		
		//insert 'world' into 'hello's subcontext map
		myContext.subContextMap.insert( ~"world", mySubContext);
		
		//insert 'hello' into the servers main context map
		myserver.contextMap.insert(~"hello", myContext);
		
		//start the server. Enjoy!
		myserver.start();
	}
}

```
