use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::BufStream;
use std::io::Error;
use std::thread;
use std::sync::Arc;

use encoder::Encoder;
use Encoders::gzip::GZIP;
use Encoders::deflate::DEFLATE;
use Encoders::chunked::CHUNKED;
use method::Method;
use Methods::GET::GET;
use request::Request;
use response::Response;

pub struct Server {
	pub ip: String,
	pub port: u16,
	pub methods_arc: Arc< HashMap< String, Method > >,
	pub encoders_arc: Arc< HashMap< String, Encoder > >
}

impl Server {

	pub fn new() -> Server {

		// build a map of Methods that we want available on the server
		let mut methods = HashMap::<String,Method>::new();
		methods.insert( "GET".to_string() , GET ) ;
		let methods_arc = Arc::new( methods );
		
		// build a map of Encoders that we want available on the server
		let mut encoders= HashMap::<String,Encoder>::new();
		encoders.insert("gzip".to_string() , GZIP );
		// we want to prefer gzip encoding over deflate, so set deflate's weight lower.
		let mut deflate = DEFLATE;
		deflate.weight = 90u8;
		encoders.insert("deflate".to_string() , deflate );
		encoders.insert("chunked".to_string() , CHUNKED ); // chunked is necessary to support 'keep-alive'
		let encoders_arc = Arc::new( encoders );
		
		// construct the server
		let server = Server { ip: "127.0.0.1".to_string(), port: 8080, methods_arc: methods_arc, encoders_arc: encoders_arc };
		return server;
	}
	
	pub fn start(&self) -> bool {
	
		let mut result = true;
		let mut address = self.ip.clone();
		address.push(':');
		address.push_str( self.port.clone().to_string().as_slice() );
		let listener = TcpListener::bind( address.as_slice() );
		
		match listener {
			Err(error) => { println!("Could not bind listener to ip: {}",error); result = false; }
			Ok(listener) => {	
				// process connections while the socket is still alive
				for stream in listener.incoming() {
					match stream {
						Err(error) => { println!("Listener error: {}",error); }
						Ok(stream) => {
							
							let thread_methods_arc = self.methods_arc.clone();
							let thread_encoders_arc = self.encoders_arc.clone();
							thread::spawn(move || {
								
								let result = Server::handle_client( stream, &*thread_methods_arc, &*thread_encoders_arc );
								match result {
									Ok(_) => {},
									Err(error) => { println!("Server handle client error: {}", error); }
								}
							});
						}
					}
				}
				// if for some reason the socket dies, drop the acceptor
				drop(listener);
			}
		}
		return result;
	}
	
	fn handle_client( stream: TcpStream, methods: &HashMap<String,Method>, encoders: &HashMap<String,Encoder>) -> Result<bool,Error> {
		let mut bufStream = BufStream::new(stream);
		
		// Keep connections alive while Keep-Alive header is present
		let mut keepAlive = true;
		
		while ( keepAlive ) {
			// Create Request
			
			let request = try!(Request::new( &mut bufStream ));
			// Build a Response from the Request
			let mut response = Response::new( &request, methods ).unwrap();
			
			response.respond( &request, encoders, &mut bufStream );
			
			// if both client and server want to keep the client alive, then do so
			match request.headers.get(&"Connection".to_string()) {
				Some(value) => 
					if ( value.as_slice() == "keep-alive" || value.as_slice() == "Keep-Alive" ) { 
						keepAlive = true;
					} else { keepAlive = false; },
				_ => { keepAlive = false; }
			}
			match response.headers.get(&"Connection".to_string()) {
				Some(value) => 
					if ( value.as_slice() == "keep-alive" || value.as_slice() == "Keep-Alive" ) { 
						keepAlive = true;
					} else { keepAlive = false; },
				_ => { keepAlive = false; }
			}
			
		}
		
		return Ok(true);
	}
}

#[test]
fn test_server() {
	// THIS ISN'T A REAL TEST, JUST A WAY TO EASILY RUN THE SERVER.
	let server = Server::new();
	server.start();
}
