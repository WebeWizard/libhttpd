#[crate_id = "httpd#0.1"];
#[crate_type="lib"];

#[allow(unnecessary_parens)];

extern mod std;
extern mod extra;
extern mod sync;

pub mod headers;
pub mod server;
pub mod context;
pub mod request;
pub mod response;
pub mod method;
pub mod requesturi;
pub mod status;

pub mod methods
{ 
	pub mod GET;
}
