use std::collections::hashmap::HashMap;
use std::io::TcpListener;
use std::io::{Acceptor, Listener};
use std::io::BufferedStream;
use std::sync::Arc;

use request::Request;
use response::Response;
use context::Context;

pub struct Server
{
	pub ip: String,
	pub port: u16,
	pub contextMap: HashMap< String, Context >
}

impl Server
{
	pub fn new() -> Server
	{
		let server: Server = Server { ip: "127.0.0.1".to_string() , port: 8080, contextMap: HashMap::< String, Context >::new() };
		return server;
	}
	
	pub fn start(&self) -> bool
	{
	
		let listener = TcpListener::bind( self.ip.as_slice() , self.port );

		// bind the listener to the specified address
		let mut acceptor = listener.listen();
		
		// clone and Arc the contextMap so we can send it to threads.
		let contextMap_arc = Arc::new( self.contextMap.clone() );

	
		// accept connections and process them, spawning a new tasks for each one
		for stream in acceptor.incoming() {
		
			// create a channel for new threads
			let (tx,rx): (Sender<Arc<HashMap<String,Context>>>,Receiver<Arc<HashMap<String,Context>>>) = channel();
			tx.send( contextMap_arc.clone() );
		
			match stream {
				Err(e) => {
					// CONNECTION FAILED
					println!("{}", e );
					
				}
				Ok(stream) => spawn(proc() {
					// CONNECTION SUCCEEDED
					
					// receive contextMap and settings for the new thread
					let localContextMap_arc = rx.recv();
					let contextMap = localContextMap_arc.deref();
					
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
								
								//search through the contexts and subcontexts to see if the uri matches any
								let mut uriSplitIter = request.uri.as_slice().split('/');
								uriSplitIter.next(); //toss the beginning / into the garbage
								let mut currentKey = uriSplitIter.next().unwrap().to_string();
								//iterate over the parts of the uri to find the deepest context
								if ( contextMap.contains_key( &currentKey ) )
								{
									let mut currentContext: &Context = contextMap.get( &currentKey );
									for key in uriSplitIter
									{
										currentKey = key.to_string();
										if ( currentContext.subContextMap.contains_key( &currentKey ) )
										{
											currentContext = currentContext.subContextMap.get( &currentKey );
										} else { break; }
									}
									//finally. perform the action of the deepest context
									(currentContext.action)( &request, &mut bufStream);
								} else {
									//if uri didn't match any context, perform the normal web server response
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
