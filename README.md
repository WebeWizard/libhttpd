Currently being worked on:
	Move responses to a structure/build/send model.  It's a library after all, not a standalone server. We shouldn't expect users to have to build contextual responses by hand to meet the http 1.1 standard.  This way, we can have functions like 'sendFileAsMessage( myfile )' and the library will send it with all the necessary headers / encodings.


Example of how to make the server, and add contexts to it

```Rust

extern mod httpd;

use std::os;

use std::io::net::ip::Ipv4Addr;
use std::hashmap::HashMap;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use httpd::server::Server;
use httpd::request::Request;
use httpd::context::Context;

//this function is the action to be performed when the 'hello' context is used ( http://mysite/hello )
fn hello( request: &Request, bufStream: &mut BufferedStream<TcpStream> )
{
	println!("hello from a Context");
}

//this function is the action to be performed when 'world', a subcontext of 'hello', is used ( http://mysite/hello/world )
fn world( request: &Request, bufStream: &mut BufferedStream<TcpStream> )
{
	println!("hello from a Sub Context!");
}

//this function is the action to be performed when the 'song' context is used ( http://mysite/song )
fn song( request: &Request, bufStream: &mut BufferedStream<TcpStream> ) 
{
	let statusLine: ~str = format!( "HTTP/1.1 {:s} {:s}\r\n", "200", "OK" );
	bufStream.write( statusLine.as_bytes() );
	//write headers ( should also do sanity checks, so we don't send content lengths with chunked encoding )
	bufStream.write( "\r\n".as_bytes() ); //end headers with an empty line
	bufStream.flush();
	
	let workingPath = os::self_exe_path().unwrap();
	let workingStr = workingPath.as_str().unwrap();
	let path = Path::new( workingStr + "/site/song.mp3" ); //from within some /site/ directory
	//send the song.mp3 file ( without headers ), the Identity transfer method is usually accompanied by a content-length header
	//but since it's the last operation we are doing, your browser knows that the end of the files occurs when the connection closes
	httpd::methods::GET::fileIdentityResponse( &path, bufStream );
}

fn main()
{
	println!("Starting web server...");
	//put it in a new thread so it doesn't block any other logic we may want outside of the web server
	do spawn {
		//create a new server with a defined ip and port
		let mut myserver: Server = Server::newFromIpAddr( Ipv4Addr(127,0,0,1), 9123 );
		
		//create the sub context 'world' struct
		let mySubContext = Context{ name: ~"world", methods: ~[], subContextMap: HashMap::<~str, Context>::new(), action: world};
		//create the main context 'hello' struct
		let mut myContext = Context{ name: ~"hello", methods: ~[], subContextMap: HashMap::<~str, Context>::new(), action: hello};
		//create the song context 'song' struct
		let mySongContext = Context{ name: ~"song", methods: ~[], subContextMap: HashMap::<~str, Context>::new(), action: song};
		
		//insert 'world' into 'hello's subcontext map
		myContext.subContextMap.insert( ~"world", mySubContext);
		
		//insert 'hello' into the servers; main context map
		myserver.contextMap.insert(~"hello", myContext);
		
		//insert 'song' into the servers' main context map
		myserver.contextMap.insert(~"song", mySongContext);
		
		//start the server. Enjoy!
		myserver.start();
	}
}

```
