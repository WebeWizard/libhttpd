use std::collections::HashMap;
use std::os;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::fs::PathExt;

use request::Request;
use response::Response;

use method::Method;
use status::Status;

// GET METHOD
pub const GET: Method = Method {name: "GET", validate: validate, build_response: build_response};

fn validate( request: &Request ) -> Status {
	// create a new absolute path using the current working directory as a base
	let mut path = PathBuf::new( &os::self_exe_path().unwrap() );
	// take off the leading '/' from the uri to make the path relative instead of absolute
	// push relative uri onto the path
	path.push( &request.uri[1..] );
	
	if ( path.is_file() ) {
		// if the path points to a file
		return Status::from_code(200).unwrap();
	//} else if ( path.is_dir() ) {
		// if the path points to a directory
		//return Status::from_code(200).unwrap();
	} else {
		return Status::from_code(404).unwrap();
	}
	
	
}

fn build_response( request: &Request ) -> Response {
	// Validate the Request to get the Status
	let status = validate( request );
	let mut messageBody: Box<Read>;
	match status.code {
		200 => {
			// create a new absolute path using the current working directory as a base
			// push the uri without the leading / to make path to file.
			let mut path = PathBuf::new( &os::self_exe_path().unwrap() );
			path.push( &request.uri[1..] );
			// open the file, and set a buffered reader to it as the messageBody
			let file: File = File::open( &path ).unwrap();
			messageBody = Box::new( BufReader::new( file ) );
		},
		_ => { messageBody = Box::new( status.reason.as_bytes() ); },
	}
	
	
	
	// ------ HEADERS -----
	let headers = HashMap::<String,String>::new();
	
	return Response { status: status , headers: headers, messageBody: messageBody };
}
