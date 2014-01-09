use std::io::net::ip::{IpAddr, SocketAddr, Ipv4Addr};
use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::buffered::BufferedStream;
use std::hashmap::HashMap;

use extra::arc::Arc;

use context::Context;

use request::Request;
use response;

pub struct Server
{
	ip: IpAddr,
	port: u16,
	settings: ~[~str],
	contextMap: HashMap<~str, Context>
}

impl Server
{	
	//Creates and returns a new Server struct with blank/default settings.
	pub fn new() -> Server
	{
		let server: Server = Server {
			ip: Ipv4Addr( 127,0,0,1 ),
			port: 9123,
			settings: ~[],
			contextMap: HashMap::<~str, Context>::new()
		};
		return server;
	}
	
	pub fn newFromPort( port: u16 ) -> Server
	{
		let server: Server = Server {
			ip: Ipv4Addr( 127,0,0,1 ),
			port: port,
			settings: ~[],
			contextMap: HashMap::<~str, Context>::new()
		};
		return server;
	}
	
	pub fn newFromIpAddr( ip: IpAddr, port: u16 ) -> Server
	{
		let server: Server = Server {
			ip: ip,
			port: port,
			settings: ~[],
			contextMap: HashMap::<~str, Context>::new()
		};
		return server;
	}
	
	//Begins the server's loop of listening for connections, building a request, and responding
	pub fn start(self) -> bool
	{

		let mut tcpAcceptor = TcpListener::bind( SocketAddr { ip: self.ip , port: self.port } ).listen().unwrap();

		println("listener is ready");
		
		let contextMap_arc = Arc::new( self.contextMap );

		loop {
			let (port,chan) = Chan::new();
			chan.send( contextMap_arc.clone() );
			let stream = tcpAcceptor.accept().unwrap();
			do spawn {
				let localArc: Arc<HashMap<~str, Context>> = port.recv();
				let contextMap = localArc.get();
				let tcpStream = stream;
				//wrap the stream in a buffer
				let mut bufStream = BufferedStream::new( tcpStream );
				
				let mut keepAlive = true;
				while ( keepAlive )
				{
					//build tcprequest from the bufStream
					let tcpRequest: Request = Request::new( &mut bufStream );
					
					if ( tcpRequest.headers.get( &~"Connection" ) == &~"close" )
					{
						keepAlive = false;
					}
					
					//search through the contexts and subcontexts to see if the uri matches any
					let mut uriSplitIter = tcpRequest.uri.split('/');
					uriSplitIter.next(); //toss the beginning / into the garbage
					let mut currentKey = uriSplitIter.next().unwrap().to_owned();
					//iterate over the parts of the uri to find the deepest context
					if ( contextMap.contains_key( &currentKey ) )
					{
						let mut currentContext: &Context = contextMap.get( &currentKey );
						for key in uriSplitIter
						{
							currentKey = key.to_owned();
							if ( currentContext.subContextMap.contains_key( &currentKey ) )
							{
								currentContext = currentContext.subContextMap.get( &currentKey );
							} else { break; }
						}
						//finally. perform the action of the deepest context
						(currentContext.action)();
					} else {
						//if uri didn't match any context, perform the normal web server response
						response::respond( &tcpRequest, &mut bufStream );
					}
				}
			}
		}
	}	
}
