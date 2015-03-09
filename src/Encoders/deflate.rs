use std::io::prelude::*;
use std::sync::mpsc::{Sender,Receiver};
use std::io::Error;
use std::io::ErrorKind::ResourceUnavailable;
use std::slice::bytes;

use flate2::read::ZlibEncoder;
use flate2::Compression;

use encoder::Encoder;

pub const DEFLATE: Encoder = Encoder {name: "deflate", weight: 100u8, encode: encode };

pub struct RecvReader {
	rx: Receiver<Vec<u8>>
}

impl Read for RecvReader {
	fn read( &mut self, buf: &mut [u8] ) -> Result<usize,Error> {
		let data = self.rx.recv();
		match ( data ) {
			Ok(realdata) => {
				bytes::copy_memory( buf, realdata.as_slice() );
				
				return Ok( realdata.len() );
			},
			Err(_) => { return Err( Error::new( ResourceUnavailable, "lol", Some("lol".to_string()))); }
		}
		
	}
}

pub fn encode ( rx: Receiver<Vec<u8>>, newtx: Sender<Vec<u8>> )
{

	// Gzip encoding using flate2 library
	let recv = RecvReader { rx: rx };
	let mut zlib = ZlibEncoder::new( recv, Compression::Default );
	const BUF_SIZE: usize = 8192;
	let mut buf = [0u8; BUF_SIZE];
		
	let mut size = BUF_SIZE;
	
	while ( size != 0 ) {
		
		match ( zlib.read( buf.as_mut_slice() ) ) {
			Ok( newsize ) => { 
				size = newsize;
				if ( size == 0 ) { break; }
				let result = newtx.send( buf[..size].to_vec() );
				match result {
					Ok(()) => {},
					Err(error) => { println!("Gzip encoder SendError: {}",error); }
				}
			},
			Err(error) => { println!("Gzip Encoder Read Error: {}",error); size = 0; }
		}
		
	}
}
