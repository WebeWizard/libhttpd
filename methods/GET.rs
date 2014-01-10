use std::os;
use std::vec;
use std::str;

use headers::Headers;
use status::Status;
use request::Request;

use response::Response;
use response::ResponseType;
use response::{FILE, DIR, ERROR};

use std::io::buffered::BufferedStream;
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
		status.statusCode = ~"404";
		status.reason = ~"Not Found";
		return ( status, ERROR );
	}
}


pub fn buildGetResponse( request: &Request ) -> Response
{
	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap();
	let path = Path::new( workingStr + request.uri );

	//see what the uri is pointing to and determine if it is gettable.
	let ( status, responseType ) = validate( request );
	
	let response: Response = Response { status: status, responseType: responseType, headers: HashMap::<~str,~str>::new() };
	return response;
	
}
//get:	fetches the data requested by the Request and sends it over the Request's bufStream.
pub fn get( request: &Request , bufStream: &mut BufferedStream<TcpStream>) -> bool
{
	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap();
	let path = Path::new( workingStr + request.uri );

	//see what the uri is pointing to and determine if it is gettable.
	let ( status, responseType ) = validate( request );
	
	//send status line
	let statusLine: ~str = format!( "HTTP/1.1 {:s} {:s}\r\n", status.statusCode, status.reason );
	bufStream.write( statusLine.as_bytes() );
	bufStream.flush();

	
	
	//send the message
	match responseType
	{
		//TODO: Move these responses into their own functions
		FILE => 
		{
			fileResponse( &path, bufStream );
		},
		DIR =>
		{
			//TODO: Check to see if an index.html file exists, if so, validate and send it instead of dir contentes
			let indexPath: Path = Path::new( workingStr + request.uri + "index.html");
			if ( indexPath.is_file() )
			{
				fileResponse( &indexPath, bufStream);
			}
			else
			{
				dirResponse( &path, bufStream );
		    	}
		},
		ERROR =>
		{
			errorResponse( &status, bufStream );
		}
	}
	return true;
}

fn fileResponse( path: &Path, bufStream: &mut BufferedStream<TcpStream> )
{
	
	let dateString = Headers::getDateHeader();
	bufStream.write( dateString.as_bytes() );
	//for persistent connections, need to include content-length header
	let contentLengthString = Headers::getContentLengthHeader( path );
	bufStream.write( contentLengthString.as_bytes() );
	//end the repsonse header with a blank line
	bufStream.write(bytes!("\r\n"));
	bufStream.flush();

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

fn dirResponse ( path: &Path, bufStream: &mut BufferedStream<TcpStream> )
{
	let dirContents = fs::readdir( path );
	let mut dirContentsResponse = ~"";
	for entry in dirContents.iter()
	{
		dirContentsResponse = dirContentsResponse + str::from_utf8( entry.filename().unwrap() ) + "\r\n";
	}
	let dirContentsBytes = dirContentsResponse.as_bytes();
	let dateString = Headers::getDateHeader();
	bufStream.write( dateString.as_bytes() );
	let contentLengthString = format!( "Content-Length: {}\r\n", dirContentsBytes.len() );
	bufStream.write( contentLengthString.as_bytes() );
	//end the repsonse header with a blank line
	bufStream.write(bytes!("\r\n"));
	bufStream.flush();
	bufStream.write( dirContentsBytes );
	bufStream.flush();
}

fn errorResponse ( status: &Status, bufStream: &mut BufferedStream<TcpStream> )
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
