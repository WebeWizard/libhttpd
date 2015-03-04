use std::collections::HashMap;
use std::old_io::net::tcp::TcpStream;
use std::old_io::BufferedStream;
use std::old_io::{BufferedReader, File};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use std::thread;

use request::Request;
use status::Status;
use method::Method;
use encoder::Encoder;
use Encoders::chunked::chunked;

use headers;

pub struct Response {
	pub status: Status,
	pub headers: HashMap< String, String >,
	pub messageBody: Box<Reader>
}

impl Response {
	//TODO: this should return a result, not an option, because method not found is valid result
	pub fn new( request: &Request, methods: &HashMap<String,Method>, encoders: &HashMap<String,Encoder> ) -> Option<Response> {
		match methods.get( &request.method ) {
			Some(method) => Some( (method.build_response)( request, encoders ) ),
			None => return None
		}
	}
	
	//TODO: return a Result instead of bool
	pub fn respond( &mut self, request: &Request, encoders: &HashMap<String,Encoder> , bufStream: &mut BufferedStream<TcpStream> ) -> bool {
		// write the status line
		let statusLine = format!( "HTTP/1.1 {} {}\r\n", self.status.code, self.status.reason );
		bufStream.write( statusLine.as_bytes() );
		
		// decide what transfer-encodings to apply (these headers are independent of the Response Method)
		let mut selected_encoders = Vec::<Encoder>::new();
		if ( request.headers.contains_key( &"Accept-Encoding".to_string() ) ) {
			let requested_encodings = request.headers.get( &"Accept-Encoding".to_string() ).unwrap();
			for requestedEncoder in requested_encodings.split_str(", ") {
				if ( requestedEncoder != "chunked" && encoders.contains_key( &requestedEncoder.to_string() ) ) {
					// for now assume we only want to use one type of encoding at a time
					selected_encoders.push( encoders.get( &requestedEncoder.to_string() ).unwrap().clone() );
				}
			}
			// if any transfer encodings are used, the last encoding must be chunked
			if ( !selected_encoders.is_empty() ) {
					selected_encoders.push( encoders.get( "chunked" ).unwrap().clone() );
			}
		}
		// if no encoders are selected, we need to use identity
		// if Content-Length has been set by Method, then we can keep the connection alive
		if ( !selected_encoders.is_empty() || ( selected_encoders.is_empty() && self.headers.contains_key("Content-Length") ) ) {
			match request.headers.get(&"Connection".to_string()) {
				Some(value) => 
					if ( value.as_slice() == "keep-alive" || value.as_slice() == "Keep-Alive") { 
						self.headers.insert( "Connection".to_string() , "Keep-Alive".to_string() );
					},
				None => ( /* connection must close */ )
			}
		}
		
	
		//build and insert transfer-encoding header
		if ( !selected_encoders.is_empty() ) {
	
			let contentKey = "Content-Encoding".to_string();
			let mut contentValue = "".to_string();
		
			for encoder in selected_encoders.iter() {
				if ( encoder.name != "chunked" ) {
					contentValue.push_str( encoder.name );
					contentValue.push(',');
				} else {
					self.headers.insert( "Transfer-Encoding".to_string(), "chunked".to_string() );
				}
				
			}
			// remove the trailing comma ','
			contentValue.pop();

			// write the line to the stream
			self.headers.insert( contentKey , contentValue );
		}
		
		
		
		
		// write the header	
		self.headers.insert( "Content-Type".to_string(), "application/octet-stream".to_string() );
		headers::write_to_stream( &self.headers , bufStream );
		
		// end the headers
		bufStream.write( "\r\n".as_bytes() );
		
		// prepare encoder threads
		let ( tx , rx ): ( Sender<Vec<u8>> , Receiver<Vec<u8>> ) = channel();
		let mut newrx = rx;
		
		for encoder in selected_encoders.iter() {
			newrx = newThread( newrx, encoder.encode );
		}
		
		// write the message
		const bufSize: usize = 8192;
		let mut buf = [0u8; bufSize];
		
		let mut size = bufSize;
		
		while ( size == bufSize ) {
			// clear the bufVec from the previous iteration
			let mut bufVec: Vec<u8> = vec![];
			// fill the buffer with new data
			match ( self.messageBody.read( &mut buf ) ) {
				Ok(readSize) => {
					// fill the bufVec with new data from buffer
					bufVec.push_all( buf.slice( 0 , readSize) );
					// send this piece of the message off to be encoded
					tx.send( bufVec );
					// this blocks the main thread while each peice is being encoded
					// this could hinder performance, maybe use a separate thread to recv and send over bufstream? Or use a separate thread to read from messageBody and send to encoders
					
					size = readSize;
				},
				_ => {}
			}
		}
		println!("done sending");
		tx.send( Vec::<u8>::new() );
		drop(tx);
		
		let mut recvIter = newrx.iter();
		for encoded in recvIter {
			bufStream.write( encoded.as_slice() );
		}
		
				
		// if there was a transfer encoding, then the last method is chunked
		// send the ending chunk and ending line
		if ( !selected_encoders.is_empty() ) {
			bufStream.write( "0\r\n\r\n".as_bytes() );
		}

		// TODO: this is very poorly done, can I do it better?
		/*
		
		*/
		
		// flush the stream
		bufStream.flush();
		
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
