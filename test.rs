extern crate httpd;

use httpd::server::Server;

fn main() {

	let server: Server = Server::new();
	
	server.start();
}
