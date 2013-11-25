use std::cell::Cell;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;

use request::Request;

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
			let mut tcpRequest: Request = Request::new(tcpStream);
			tcpRequest.respond();
		}
	}
}
