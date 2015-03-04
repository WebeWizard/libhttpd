extern crate flate;

use encoder::Encoder;

// TODO: The flate crate doesn't work for this application yet.
pub const deflate: Encoder = Encoder {name: "deflate", encode: encode};

pub fn encode( bufVec: Vec<u8> ) -> Vec<u8>
{
	let mut test: Vec<u8> = vec![];
	let deflated_bytes = &*flate::deflate_bytes( bufVec.as_slice() ).unwrap();
	test.push_all( deflated_bytes );
	return test;
	
}
