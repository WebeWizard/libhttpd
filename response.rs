use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;
use std::hashmap::HashMap;

use request::Request;
use method::{ CONNECT,DELETE,GET,HEAD,OPTIONS,POST,PUT,TRACE };
use methods::GET;
use status::Status;

pub enum ResponseType
{
	FILE,
	DIR,
	ERROR
}

pub struct Response
{
	status: Status,
	responseType: ResponseType,
	headers: HashMap<~str, ~str>,
}

	
//respond:	Decides how to respond to the request, and then does so.
pub fn respond( request: &Request , bufStream: &mut BufferedStream<TcpStream> ) -> bool
{
	//TODO: Move to a response-builder / response-sender model
	let mut successFlag: bool;
	 match request.method
        {
                CONNECT =>
                {
                        //NOT IMPLEMENTED YET
                        println("CONNECT is not implemented yet");
                        successFlag = false;
                },
                DELETE =>
                {
                        //NOT IMPLEMENTED YET
                        successFlag = false;
                },
                GET =>
                {
                	successFlag = GET::get( request, bufStream );
                },
                HEAD =>
                {
                        //NOT IMPLEMENTED YET
                        successFlag = false;
                },
                OPTIONS =>
                {
                        //NOT IMPLEMENTED YET
                        successFlag = false;
                },
                POST =>
                {
                        //NOT IMPLEMENTED YET
                        successFlag = false;
                },
                PUT =>
                {
                        //NOT IMPLEMENTED YET
                        successFlag = false;
                },
                TRACE =>
                {
                        //NOT IMPLEMENTED YET
                        successFlag = false;
                },
        }
        return successFlag;
}

