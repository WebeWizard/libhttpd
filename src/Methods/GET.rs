use std::collections::HashMap;
use std::os;
use std::old_io::BufferedReader;
use std::old_io::fs;
use std::old_io::File;
use std::old_io::fs::PathExtensions;

use request::Request;
use response::Response;

use method::Method;
use status::Status;
use encoder::Encoder;

// GET METHOD
pub const GET: Method = Method {name: "GET", validate: validate, build_response: build_response};

pub enum ResponseType {
	FILE,
	DIR,
	ERROR
}

fn validate( request: &Request ) -> Status {
	let uri = request.uri.as_slice();
	let workingPath = os::self_exe_path().unwrap();
	let mut workingStr = workingPath.as_str().unwrap().to_string();
	workingStr.push_str( uri );
	let path = Path::new( workingStr );
	
	//println!("{}",path.display() );
	
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

fn build_response( request: &Request, encoders: &HashMap<String,Encoder> ) -> Response {
	// Validate the Request to get the Status
	let status = validate( request );
	
	let uri = request.uri.as_slice();
	let workingPath = os::self_exe_path().unwrap();
	let mut workingStr = workingPath.as_str().unwrap().to_string();
	workingStr.push_str( uri );
	let path = Path::new( workingStr );
	let mut file: File = File::open( &path ).unwrap();
	let mut messageBody = Box::new( BufferedReader::new( file ) );
	
	
	// ------ HEADERS -----
	let mut headers = HashMap::<String,String>::new();
	//headers.insert("Content-Length".to_string(),"4".to_string());
	
	return Response { status: status , headers: headers, messageBody: messageBody };
}
