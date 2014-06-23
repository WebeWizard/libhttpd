Currently being worked on:
	- Contexts should only act on the methods defined in their methods vector.	
	- Re-add directory responses and default index.html to get request.
	- gzip and deflate encoding.


Compiling:
	Compile using 'rustc lib.rs' , that's it.
	
	
Usage:

```Rust

extern crate httpd;

use std::collections::hashmap::HashMap;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use httpd::server::Server;
use httpd::request::Request;
use httpd::context::Context;

fn hello( request: &Request, bufStream: &mut BufferedStream<TcpStream> )
{
	println!("hello from a Context");
}

fn main() {

	let mut server: Server = Server::new();
	
	server.contextMap.insert("test".to_string(), Context{ methods: vec![], subContextMap: HashMap::<String,Context>::new() , action: hello } );
	
	server.start();
}

```
