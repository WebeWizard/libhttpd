use std::os;

use status::Status;


//validate:	validates the requested path and returns the appropriate status 
//TODO:		how about the server only serving files from something like a 'www' folder?
//TODO:		should probably add permission checks, so the server doesn't try to access files it isn't supposed to
pub fn validate( requestedStr: &str ) -> Status
{
	let mut status: Status = Status { statusCode: ~"500", reason: ~"Internal Server Error" };

	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap();
	let path = Path::init( workingStr + requestedStr );
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

