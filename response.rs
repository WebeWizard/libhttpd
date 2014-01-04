use std::io::buffered::BufferedStream;
use std::io::net::tcp::TcpStream;

use request::Request;

use method::{ CONNECT,DELETE,GET,HEAD,OPTIONS,POST,PUT,TRACE };
use methods::GET;


	
//respond:	Decides how to respond to the request, and then does so.
pub fn respond( request: &Request , bufStream: &mut BufferedStream<TcpStream> ) -> bool
{
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

