use std::collections::hashmap::HashMap;

use std::os;
use std::io::fs;

use request::Request;
use response::{Response, ResponseType};
use response::{FILE,DIR,ERROR};
use status::Status;
use encoder::Encoder;
use encoders::{identity,chunked};
use sender;

pub fn response( request: &Request ) -> Response
{
	let mut headers = HashMap::<String,String>::new();

	// validate the request, make sure it actually points to something gettable
	let ( statusCode, responseType ) = validate( request );
	
	// build a function for responding with
	let mut encoder: Encoder = Encoder{ encoders: vec![] };
	let mut messageSender = sender::nothing_sender;
	
	match responseType
	{
		FILE =>{
			// Transfer Encodings
			let mut transferEncodingsVec: Vec<&str> = vec![];
			match ( request.headers.find( &"transfer-encoding".to_string() ) ) {
				Some( encodings ) => {
					let availableEncodings: Vec<&str> = encodings.as_slice().split(',').collect();
					// if gzip is available, then gzip it.
					if ( availableEncodings.contains( &"gzip") )
					{
						//transferEncodingsVec.push( "gzip" );
						
					}
					// if any transfer encoding is applied, then the very last encoding must be chunked
					transferEncodingsVec.push( "chunked" );
				},
				None => {
					let uri = request.uri.as_slice();
					let workingPath = os::self_exe_path().unwrap();
					let workingStr = workingPath.as_str().unwrap().to_string();
					let path = Path::new( workingStr.append( uri ) );
					let contentLength = fs::stat( &path ).unwrap().size;
					headers.insert( "content-length".to_string() , format!("{}",contentLength) );
					transferEncodingsVec = vec![ "identity" ];
				}
			}
			// add the transfer encodings to the headers
			let transferEncodingString = transferEncodingsVec.connect(",");
			if ( transferEncodingString.as_slice() != "identity" )
			{
				headers.insert( "transfer-encoding".to_string() , transferEncodingString );
			}
			
			// build the encoder
			for newEncode in transferEncodingsVec.iter()
			{
				match newEncode
				{
					&"identity" => { 
						encoder.encoders.push( identity::identity );
					},
					&"chunked" => {
						encoder.encoders.push( chunked::chunk );
					},
					_ => {}
				}
			}

			// set the message sender to be file_sender
			messageSender = sender::file_sender;
			
		},
		DIR => {
			
		},
		ERROR => {
			encoder.encoders.push( identity::identity );
			messageSender = sender::error_sender;
		}
	}
	
	//build the Response struct
	let status = Status::from_code( statusCode );
	let response = Response{ status: status, responseType: responseType, encoder: encoder, headers: headers, messageSender: messageSender };

	return response;
}

// check to make sure that the URI points to a valid file or directory.
// TODO: Need to check other headers like 'if-modified-since'
pub fn validate( request: &Request ) -> ( u16 , ResponseType )
{
	let uri = request.uri.as_slice();
	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap().to_string();
	let path = Path::new( workingStr.append( uri ) );
	
	if ( path.is_file() ){
		return ( 200, FILE );
	}
	
	if ( path.is_dir() )
	{
		return ( 200, DIR );
	}
	
	//not a file or a directory, not found or path error
	return ( 404, ERROR );
}
