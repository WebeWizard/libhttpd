use std::os;

use status::Status;
use request::Request;

use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;


//validate:	validates the requested path and returns the appropriate status 
//TODO:		how about the server only serving files from something like a 'www' directory?
//TODO:		should probably add permission checks, so the server doesn't try to access files it isn't supposed to
pub fn validate( requestedStr: &str ) -> Status
{
	let mut status: Status = Status { statusCode: ~"500", reason: ~"Internal Server Error" };

	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap();
	let path = Path::new( workingStr + requestedStr );
	if ( path.exists() )
	{
		status.statusCode = ~"200";
		status.reason = ~"OK";
	}
	else
	{
		status.statusCode = ~"404";
		status.reason = ~"Not Found";
	}
	return status;
	
}

//get:	fetches the data requested by the Request and sends it over the Request's bufStream.
pub fn get( request: &Request , bufStream: &mut BufferedStream<TcpStream>) -> bool
{
	//validate the uri and see if it is gettable
	let status: Status = validate( request.uri );
	
	//send status line
	let statusLine: ~str = format!( "HTTP/1.1 {:s} {:s}\r\n", status.statusCode, status.reason );
	bufStream.write( statusLine.as_bytes() );
	bufStream.flush();
	
	//TODO: send response headers
	
	
	//end the repsonse header with a blank line
	bufStream.write(bytes!("\r\n"));
	bufStream.flush();
	
	//send the message
	bufStream.write(bytes!("poop"));
	bufStream.flush();
	
	
	return true;
}
