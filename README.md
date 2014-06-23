Currently being worked on:
	Completely re-wrote the library.  It's still following Rust nightly ( 0.11-pre ).
	Still need to add contexts back, but there have been many improvements, like http keep-alive, and a better base for transfer/content encoding.
	
	Re-add directory responses and default index.html to get request.
	Being worked on now: gzip and deflate encoding.


Compiling:
	Compile using 'rustc lib.rs' , that's it.
	
	
Usage:

```Rust

extern crate httpd;

use httpd::server::Server;

fn main() {

	//create the default server.  Default port is 9123
	let server: Server = Server::new();
	
	server.start();
}

```
