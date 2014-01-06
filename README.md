Example of how to make the server, and add contexts to it

```Rust

extern mod httpd;

use httpd::server::Server;
use httpd::context::Context;

fn hi()
{
	println!("hello from a context");
}

fn main()
{
	println("Starting web server...");
	do spawn {
		let mut myserver: Server = Server::new();
		let myContext = Context{ name: ~"/hi.txt", methods: ~[], subContexts: ~[], action: hi};
	
		myserver.contextMap.insert(~"/hi.txt", myContext);
		myserver.start();
	}
}

```
