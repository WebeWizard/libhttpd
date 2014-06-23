use std::os;
use std::io::File;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use request::Request;
use response::Response;

// A Nothing Sender, the default for new responses, sends nothing
pub fn nothing_sender( request: &Request, response: &Response, bufStream: &mut BufferedStream<TcpStream> ) -> bool {return true;}

// A File Sender, sends the contents of the file specified by the Request uri.
pub fn file_sender( request: &Request, response: &Response, bufStream: &mut BufferedStream<TcpStream> ) -> bool
{
	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap().to_string();
	let path = Path::new( workingStr.append( request.uri.as_slice() ) );
	let mut file: File = File::open( &path ).unwrap();
	let mut buf = [0u8, ..8192];
	let mut bufVec: Vec<u8> = vec![];
	while ( !file.eof() )
	{
		match file.read(buf)
		{
			Ok( size ) =>
			{
				bufVec = Vec::from_slice( buf.slice( 0, size ) );

				bufStream.write( response.encoder.encode( bufVec ).as_slice() );
				//gzip
			},
			_ =>
			{
				break;
			}
		}
	}
	return true;
}

// An Error Sender, sends a brief error message
pub fn error_sender( request: &Request, response: &Response, bufStream: &mut BufferedStream<TcpStream> ) -> bool {
	bufStream.write( format!("ERROR: {} - {}", response.status.code , response.status.reason ).as_bytes() );
	return true;
}
