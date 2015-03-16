use std::io::prelude::*;
use std::sync::mpsc::{Sender,Receiver};
use std::io::Error;
use std::io::ErrorKind::ResourceUnavailable;
use std::slice::bytes;

use flate2::read::GzEncoder;
use flate2::Compression;

use encoder::Encoder;

pub const GZIP: Encoder = Encoder {name: "gzip", weight: 100u8, encode: encode };

pub struct RecvReader {
	rx: Receiver<Vec<u8>>,
	eof: bool
}

impl Read for RecvReader {
	fn read( &mut self, buf: &mut [u8] ) -> Result<usize,Error> {
		if ( !self.eof ) {
			let data = self.rx.recv();
			match ( data ) {
				Ok(realdata) => {
					bytes::copy_memory( buf, realdata.as_slice() );
					if ( realdata.len() == 0 ) { self.eof = true; }
					return Ok( realdata.len() );
				},
				Err(_) => { return Err( Error::new( ResourceUnavailable, "Gzip RecvReader Error", Some("Gzip encoder: Error trying to read from channel receiver".to_string()))); }
			}
		} else { return Ok(0); }
		
	}
}

pub fn encode ( rx: Receiver<Vec<u8>>, newtx: Sender<Vec<u8>> )
{
	// Gzip encoding using flate2 library
	let recv = RecvReader { rx: rx, eof: false };
	let mut gz = GzEncoder::new( recv, Compression::Default );
	const BUF_SIZE: usize = 8192;
	let mut buf = [0u8; BUF_SIZE];
		
	let mut size = BUF_SIZE;
	
	while ( size != 0 ) {
		match ( gz.read( buf.as_mut_slice() ) ) {
			Ok( newsize ) => { 
				size = newsize;
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
