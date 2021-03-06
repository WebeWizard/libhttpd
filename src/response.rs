use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufStream,Read,Write};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use std::thread;

use request::Request;
use status::Status;
use method::Method;
use encoder::Encoder;

use headers;

pub struct Response {
	pub status: Status,
	pub headers: HashMap< String, String >,
	pub messageBody: Box<Read>
}

impl Response {
	//TODO: this should return a result, not an option, because method not found is valid result
	pub fn new( request: &Request, methods: &HashMap<String,Method> ) -> Option<Response> {
		match methods.get( &request.method ) {
			Some(method) => Some( (method.build_response)( request ) ),
			None => return None
		}
	}
	
	//TODO: return a Result instead of bool
	pub fn respond( &mut self, request: &Request, encoders: &HashMap<String,Encoder> , bufStream: &mut BufStream<TcpStream> ) -> bool {
		// write the status line
		let statusLine = format!( "HTTP/1.1 {} {}\r\n", self.status.code, self.status.reason );
		let result = bufStream.write_all( statusLine.as_bytes() );
		match result {
			Ok(()) => {},
			Err(error) => { println!("Stream write error: {}", error); }
		}
		
		// decide what transfer-encodings to apply (these headers are independent of the Response Method)
		let mut selected_encoders = Vec::<Encoder>::new();
		if ( request.headers.contains_key( &"accept-encoding".to_string() ) ) {
			let requested_encodings = request.headers.get( &"accept-encoding".to_string() ).unwrap();
			let mut weight = 0u8;
			for requestedEncoder in requested_encodings.split(", ") {
				if ( requestedEncoder != "chunked" && encoders.contains_key( &requestedEncoder.to_string() ) ) {
					let this_encoder = encoders.get( &requestedEncoder.to_string() ).unwrap().clone();
					// if this encoder has more weight, clear the vec
					if ( this_encoder.weight > weight ) {
						selected_encoders.clear();
						weight = this_encoder.weight;
						selected_encoders.push( this_encoder );
					}
					// if this encoder is equal weight, add it to the end of the vec
					else if ( this_encoder.weight == weight ) {
						selected_encoders.push( this_encoder );
					}
				}
			}
			// if any transfer encodings are used, the last encoding must be chunked
			if ( !selected_encoders.is_empty() ) {
					selected_encoders.push( encoders.get( "chunked" ).unwrap().clone() );
			}
		}
		// if no encoders are selected, we need to use identity
		// if Content-Length has been set by Method, then we can keep the connection alive
		if ( !selected_encoders.is_empty() || ( selected_encoders.is_empty() && self.headers.contains_key("content-length") ) ) {
			match request.headers.get("connection") {
				Some(value) => 
					if ( value.as_slice() == "keep-alive" || value.as_slice() == "Keep-Alive") { 
						self.headers.insert( "connection".to_string() , "keep-alive".to_string() );
					},
				None => ( /* connection must close */ )
			}
		}
		
	
		//build and insert transfer-encoding header
		if ( !selected_encoders.is_empty() ) {
	
			let contentKey = "content-encoding".to_string();
			let mut contentValue = "".to_string();
		
			for encoder in selected_encoders.iter() {
				if ( encoder.name != "chunked" ) {
					contentValue.push_str( encoder.name );
					contentValue.push(',');
				} else {
					self.headers.insert( "transfer-encoding".to_string(), "chunked".to_string() );
				}
				
			}
			// remove the trailing comma ','
			contentValue.pop();

			// write the line to the stream
			self.headers.insert( contentKey , contentValue );
		}
		
		
		
		
		// write the header
		headers::write_to_stream( &self.headers , bufStream );
		
		// end the headers
		let result = bufStream.write_all( "\r\n".as_bytes() );
		match result {
			Ok(()) => {},
			Err(error) => { println!("Stream write error: {}", error); }
		}
		
		// prepare encoder threads
		let ( tx , rx ): ( Sender<Vec<u8>> , Receiver<Vec<u8>> ) = channel();
		let mut newrx = rx;
		
		for encoder in selected_encoders.iter() {
			newrx = newThread( newrx, encoder.encode );
		}
		
		// encode the message
		const BUF_SIZE: usize = 8192;
		let mut buf = [0u8; BUF_SIZE];
		let mut size = BUF_SIZE;
		
		while ( size != 0 ) {
			// clear the bufVec from the previous iteration
			let mut bufVec: Vec<u8> = vec![];
			// fill the buffer with new data
			match ( self.messageBody.read( &mut buf ) ) {
				Ok(readSize) => {
					// fill the bufVec with new data from buffer
					bufVec.push_all( &buf[..readSize] );
					// send this piece of the message off to be encoded
					let result = tx.send( bufVec );
					match result {
						Ok(()) => {},
						Err(error) => { println!("Encoders sender error: {}", error); }
					}
					
					size = readSize;
				},
				_ => {}
			}
		}
		
		// read and send the encoded message
		size = BUF_SIZE;
		while ( size != 0 ) {
			match ( newrx.recv( ) ) {
				Ok( data ) => { 
					size = data.len();
					//println!("{}",size);
					let result = bufStream.write_all( data.as_slice() );
					match result {
						Ok(()) => {},
						Err(error) => { println!("Stream write error: {}", error); }
					}
				},
				Err(error) => { println!("Response Encoders Read Error: {}",error); size = 0; }
			}
		
		}
				
		// if there was a transfer encoding, then the last method is chunked
		// send the ending chunk and ending line
		if ( !selected_encoders.is_empty() ) {
			let result = bufStream.write_all( "0\r\n\r\n".as_bytes() );
			match result {
				Ok(()) => {},
				Err(error) => { println!("Stream write error: {}", error); }
			}
		}

		
		// flush the stream
		let result = bufStream.flush();
		match result {
			Ok(()) => {},
			Err(error) => { println!("Stream flush error: {}", error); }
		}

		return true;
		
	
	}
}

fn newThread ( rx: Receiver<Vec<u8>>, f: fn( Receiver<Vec<u8>>, Sender<Vec<u8>> ) ) -> Receiver<Vec<u8>> {
	let ( newtx , newrx ): ( Sender<Vec<u8>> , Receiver<Vec<u8>> ) = channel();
	thread::spawn( move || {
		f( rx, newtx );
	} );

	return newrx;

} 
