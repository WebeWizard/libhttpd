use std::os;

use status::Status;
use request::Request;

use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;

use std::io::fs;

pub enum ResponseType
{
	FILE,
	DIR,
	ERROR
}
	
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
	
	//TODO: send response headers
	
	
	//end the repsonse header with a blank line
	bufStream.write(bytes!("\r\n"));
	bufStream.flush();
	
	//send the message
	match responseType
	{
		//TODO: Move these responses into their own functions
		FILE => 
		{
			
			bufStream.write(bytes!("I am a file"));
			bufStream.flush();
		},
		DIR =>
		{
			//TODO: Check to see if an index.html file exists, if so, validate and send it instead of dir contentes
			let dirContents = fs::readdir(&path);
			for entry in dirContents.iter()
			{
            			bufStream.write( entry.filename().unwrap() + bytes!("\r\n") );
            			bufStream.flush();
            		}	
		},
		ERROR =>
		{
			let errorLine: ~str = format!( "ERROR: {:s} , {:s} ", status.statusCode, status.reason);
			bufStream.write( errorLine.as_bytes() );
			bufStream.flush();
		}
	}
	return true;
}
