use std::cell::Cell;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::buffered::BufferedStream;

use request::Request;
use response;

pub fn start()
{
	//static bindAddress: &'static str = "127.0.0.1";
	//static bindPort: uint = 9123;
	
	let mut tcpAcceptor = TcpListener::bind( SocketAddr { ip: Ipv4Addr(127,0,0,1) , port: 9123 } ).listen().unwrap();
	//println( format!("Server is listening on IP: {:s} , Port: {:u}", bindAddress, bindPort) );
	
	println("listener is ready");
	
	loop {
		let stream = Cell::new( tcpAcceptor.accept().unwrap() );
		do spawn {
			let tcpStream = stream.take();
			//wrap the stream in a buffer
			let mut bufStream = BufferedStream::new( tcpStream );
			//build tcprequest from the bufStream
			let tcpRequest: Request = Request::new( &mut bufStream );
			//respond to the request
			response::respond( &tcpRequest, &mut bufStream );
		}
	}
}
