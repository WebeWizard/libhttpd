use std::io::TcpListener;
use std::io::{Acceptor, Listener};
use std::io::BufferedStream;

use request::Request;
use response::Response;

pub struct Server<'s>
{
	ip: &'s str,
	port: u16
}

impl<'s> Server<'s>
{
	pub fn new() -> Server
	{
		let server: Server = Server { ip: "127.0.0.1", port: 8080 };
		return server;
	}
	
	pub fn start(&self) -> bool
	{
	
		let listener = TcpListener::bind( self.ip, self.port );

		// bind the listener to the specified address
		let mut acceptor = listener.listen();

	
		// accept connections and process them, spawning a new tasks for each one
		for stream in acceptor.incoming() {
			match stream {
				Err(e) => {
					// CONNECTION FAILED
					println!("{}", e );
					
				}
				Ok(stream) => spawn(proc() {
					// CONNECTION SUCCEEDED
					
					// buffer the stream
					let mut bufStream = BufferedStream::new( stream );
				
					let mut keepAlive: bool = true;
					while ( keepAlive )
					{
						// Read Request or Catch Read Error
						let requestOption = Request::new( &mut bufStream );
						match requestOption
						{	
							// Request was successfully read
							Some(request) => {
								// Read Connection header for keep-alive
								if ( request.headers.contains_key(&"Connection".to_string()) )
								{
									let value = request.headers.get(&"Connection".to_string());
									if ( value.as_slice() != "Keep-Alive" ) { keepAlive = false;}
								}
								else { keepAlive = false; }
				
								// Respond
								let responseOption = Response::new( &request );
								match responseOption
								{
									// A valid response was created
									Some(mut response) => {
										response.respond( &request, &mut bufStream );
									},
									// Response creation failed, maybe do some logging?
									None => {}
								}
							},
							// Read Request failed, connection probably closed
							None => { break; }
						}
					}
				})
			}
		}
		// close the socket server
		drop(acceptor);
		
		return true;
	}
}
