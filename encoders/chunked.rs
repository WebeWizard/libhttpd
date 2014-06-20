use std::num::ToStrRadix;

pub fn chunk( bufVec: Vec<u8> ) -> Vec<u8>
{
	let mut hexSizeStr = bufVec.len().to_str_radix(16);
	hexSizeStr.push_str( "\r\n" );
	let mut chunk: Vec<u8> = vec![];
	chunk.push_all( hexSizeStr.as_bytes() );
	chunk.push_all( bufVec.as_slice() );
	chunk.push_all( "\r\n".as_bytes() );
	
	return chunk;
}
