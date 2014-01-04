use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::buffered::BufferedStream;

use context::Context;

use request::Request;
use response;

pub struct Server
{
	settings: ~[~str],
	contexts: ~[Context]
}

impl Server
{	
	//Creates and returns a new Server struct with blank/default settings.
	pub fn new() -> Server
	{
		let blankSettings: ~[~str] = ~[];
		let blankContexts: ~[Context] = ~[];
		let server: Server = Server { settings: blankSettings, contexts: blankContexts };
		return server;
	}
	
	//Begins the server's loop of listening for connections, building a request, and responding
	pub fn start(&self) -> bool
	{
		do spawn {
			//static bindAddress: &'static str = "127.0.0.1";
			//static bindPort: uint = 9123;

			let mut tcpAcceptor = TcpListener::bind( SocketAddr { ip: Ipv4Addr(127,0,0,1) , port: 9123 } ).listen().unwrap();
			//println( format!("Server is listening on IP: {:s} , Port: {:u}", bindAddress, bindPort) );

			println("listener is ready");

			loop {
				let stream = tcpAcceptor.accept().unwrap();
				do spawn {
					let tcpStream = stream;
					//wrap the stream in a buffer
					let mut bufStream = BufferedStream::new( tcpStream );
					//build tcprequest from the bufStream
					let tcpRequest: Request = Request::new( &mut bufStream );
					//respond to the request
					response::respond( &tcpRequest, &mut bufStream );
				}
			}
		}
		return true;
	}	
}
