use std::os;
use std::vec;
use std::str;

use headers::Headers;
use status::Status;
use request::Request;

use response::Response;
use response::ResponseType;
use response::{FILE, DIR, ERROR};

use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use std::hashmap::HashMap;

use std::io::{File, fs};

//validate:	validates the request from the uri and headers 
//		and determines how the server should respond by returning a Status struct.
//TODO:		how about the server only serving files from something like a 'www' directory?
//TODO:		should probably add permission checks, so the server doesn't try to access files it isn't supposed to
pub fn validate( request: &Request ) -> ( Status, ResponseType )
{
	let mut status: Status = Status { statusCode: ~"500", reason: ~"Internal Server Error" };

	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap();
	let path = Path::new( workingStr + request.uri );
	
	if ( path.is_file() )
	{
		status.statusCode = ~"200";
		status.reason = ~"OK";
		return ( status, FILE );
	}
	
	if ( path.is_dir() )
	{
		status.statusCode = ~"200";
		status.reason = ~"OK";
		return ( status, DIR );
	}
	
	else
	{
		//TODO: Build this more to include different types of errors (Unauthorized, Gone, Request-URI Too Large(?) etc)
		status.statusCode = ~"404";
		status.reason = ~"Not Found";
		return ( status, ERROR );
	}
}


pub fn buildGetResponse( request: &Request ) -> Response
{
	let mut headers = HashMap::<~str,~str>::new();
	//see what the uri is pointing to and determine if it is gettable.
	let ( status, responseType ) = validate( request );
	//decide what transfer encoding we want to use
	match responseType
	{
		FILE => 
		{
			
			if ( false ) //if none of these rules match, don't enter any header, and we'll use Identity encoding as a last resort
			{
				headers.insert( ~"Transfer-Encoding" , ~"chunked" );
			} 
			else
			{
				let workingPath = os::self_exe_path().unwrap();
				let workingStr = workingPath.as_str().unwrap();
				let path = Path::new( workingStr + request.uri );
				let size = fs::stat( &path ).size;
				headers.insert( ~"Content-Length", size.to_str() );
			}
		},
		DIR =>
		{
			let workingPath = os::self_exe_path().unwrap();
			let workingStr = workingPath.as_str().unwrap();
			let indexPath: Path = Path::new( workingStr + request.uri + "index.html");
			if ( indexPath.is_file() )
			{
				let size = fs::stat( &indexPath ).size;
				headers.insert( ~"Content-Length", size.to_str() );
			}
			else
			{
				//we used chunked for this because we never know what's in a directory ahead of time
				headers.insert( ~"Transfer-Encoding" , ~"chunked" );
			}
			//In the future, we'll use chunked for this, because we never know what's in a directory ahead of time
			
		},
		ERROR =>
		{
			//for now, let's always use identiy for this, until we know better anyways
		}
	}

	
	let response: Response = Response { status: status, responseType: responseType, headers: headers };
	return response;
}

pub fn sendGetResponse( request: &Request, response: &Response, bufStream: &mut BufferedStream<TcpStream> )
{
	//write status line
	let statusLine: ~str = format!( "HTTP/1.1 {:s} {:s}\r\n", response.status.statusCode, response.status.reason );
	bufStream.write( statusLine.as_bytes() );
	//write headers ( should also do sanity checks, so we don't send content lengths with chunked encoding )
	Headers::writeToStream( &response.headers , bufStream);
	bufStream.write( "\r\n".as_bytes() ); //end headers with an empty line
	bufStream.flush();
	
	//write message
	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap();
	let path = Path::new( workingStr + request.uri );
	match response.responseType
	{
		FILE => 
		{
			fileIdentityResponse( &path, bufStream );
		},
		DIR =>
		{
			let indexPath: Path = Path::new( workingStr + request.uri + "index.html");
			if ( indexPath.is_file() )
			{
				fileIdentityResponse( &indexPath, bufStream );
			}
			else
			{
				//In the future, we'll use chunked for this, because we never know what's in a directory ahead of time
				dirChunkedResponse( &path, bufStream );
			}
		},
		ERROR =>
		{
			errorIdentityResponse( &response.status, bufStream );
		}
	}

	//if headers contains a Trailers field, then send trailers
	//flush
}
//get:	fetches the data requested by the Request, builds a response, and sends it over the Request's bufStream.
pub fn get( request: &Request , bufStream: &mut BufferedStream<TcpStream>) -> bool
{
	let response: Response = buildGetResponse( request );
	sendGetResponse( request, &response, bufStream );
	return true;
}

fn fileIdentityResponse( path: &Path, bufStream: &mut BufferedStream<TcpStream> )
{
	//IDENTITY ENCODING FOR FILES ( send the file as is with Content-Length in header
	let mut file: File = File::open( path ).unwrap();
	let mut buf  = vec::from_elem(8129, 0u8);
	while ( !file.eof() )
	{
		match file.read(buf)
		{
			Some(length) =>
			{
				bufStream.write( buf.mut_slice( 0, length) );
				bufStream.flush();
			},
			None => { break; }
		}
	}
}

fn fileChunkedResponse ( path: &Path, bufStream: &mut BufferedStream<TcpStream> )
{
	//CHUNKED ENCODING for sending files, this should be the last of the last resort methods for sending files
	let mut file: File = File::open( path ).unwrap();
	let mut buf = vec::from_elem(8129, 0u8);
	while ( !file.eof() )
	{
		match file.read(buf)
		{
			Some(length) =>
			{
				let hexSizeStr = length.to_str_radix(16);
				let sizeStr = hexSizeStr + "\r\n"; //INCLUDING ending CRLF
				bufStream.write( sizeStr.as_bytes() );
				bufStream.write( buf.mut_slice( 0, length) );
				bufStream.write( "\r\n".as_bytes() );
				bufStream.flush();
			},
			None => { break; }
		}
	}
	bufStream.write( "0\r\n\r\n".as_bytes() ); //end the chunked data
	bufStream.flush(); 
}

fn dirChunkedResponse ( path: &Path, bufStream: &mut BufferedStream<TcpStream> )
{
	//CHUNKED ENCODING for directory listing ( Each chunk contains a hex length of chunk message, followed by CRLF, followed by chunk message, followed by CRLF )
	let dirContents = fs::readdir( path );
	for entry in dirContents.iter()
	{
		let entryStr = str::from_utf8( entry.filename().unwrap() ).unwrap() + "\r\n"; //NOT including ending CRLF
		let hexSizeStr = entryStr.as_bytes().len().to_str_radix(16);
		let sizeStr = hexSizeStr + "\r\n"; //INCLUDING ending CRLF
		bufStream.write( sizeStr.as_bytes() );
		bufStream.write( entryStr.as_bytes() );
		bufStream.write( "\r\n".as_bytes() );
		bufStream.flush();
	}
	bufStream.write( "0\r\n\r\n".as_bytes() ); //end the chunked data
	bufStream.flush(); 
}

fn errorIdentityResponse ( status: &Status, bufStream: &mut BufferedStream<TcpStream> )
{
	let dateString = Headers::getDateHeader();
	bufStream.write( dateString.as_bytes() );
	let errorLine: ~str = format!( "ERROR: {:s} , {:s} ", status.statusCode, status.reason);
	let contentLengthString = format!( "Content-Length: {}\r\n", errorLine.len() );
    	bufStream.write( contentLengthString.as_bytes() );
    	//end the repsonse header with a blank line
    	bufStream.write(bytes!("\r\n"));
	bufStream.flush();
	bufStream.write( errorLine.as_bytes() );
	bufStream.flush();
}
