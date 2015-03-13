
use std::fmt::radix;
use std::sync::mpsc::{Sender,Receiver};

use encoder::Encoder;

// GLOBAL STATIC CHUNKED ENCODER
pub const CHUNKED: Encoder = Encoder {name: "chunked", weight: 100u8, encode: encode};

pub fn encode ( rx: Receiver<Vec<u8>>, newtx: Sender<Vec<u8>> )
{

	let mut size = 8192;
	while ( size != 0 ) { 
		let data = rx.recv();
		match ( data ) {
			Ok( realdata ) => {
				size = realdata.len();
				let mut hexSizeStr = radix(realdata.len(),16).to_string();
				hexSizeStr.push_str( "\r\n" );
				let mut chunk: Vec<u8> = vec![];
				chunk.push_all( hexSizeStr.as_bytes() );
				chunk.push_all( realdata.as_slice() );
				chunk.push_all( "\r\n".as_bytes() );
				let result = newtx.send( chunk );
				match result {
					Ok(()) => {},
					Err(error) => { println!("Chunked encoder SendError: {}",error); }
				}
			},
			Err(_) => { break; }
		}
	}

}
