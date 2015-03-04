use std::io::prelude::*;
use std::old_io::ChanReader;
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::RecvError;
use std::str;
use std::io::Error;
use std::io::ErrorKind::ResourceUnavailable;
use std::slice::bytes;

use flate2::read::GzEncoder;
use flate2::Compression;

use encoder::Encoder;

// TODO: The flate crate doesn't work for this application yet.
pub const gzip: Encoder = Encoder {name: "gzip", encode: encode };

pub struct test {
	rx: Receiver<Vec<u8>>
}

impl Read for test {
	fn read( &mut self, buf: &mut [u8] ) -> Result<usize,Error> {
		let mut data = self.rx.recv();
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
	let mut test = test { rx: rx };
	let mut gz = GzEncoder::new( test, Compression::Default );
	const bufSize: usize = 8192;
	let mut buf = [0u8; bufSize];
		
	let mut size = bufSize;
	
	while ( size != 0 ) {
		
		match ( gz.read( buf.as_mut_slice() ) ) {
			Ok( newsize ) => { 
				size = newsize;
				if ( size == 0 ) { break; }
				newtx.send( buf[..size].to_vec() );
			},
			Err(error) => { size = 0; }
		}
		
	}
}
