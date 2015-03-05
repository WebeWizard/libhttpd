use request::Request;
use response::Response;
use status::Status;

pub struct Method {
	pub name: &'static str,
	pub validate: fn ( &Request ) -> Status,
	pub build_response: fn ( &Request ) -> Response
}


impl Clone for Method {
	fn clone(&self) -> Method {
		return Method {
			name : self.name.clone(),
			validate : self.validate,
			build_response: self.build_response
		}
	}
}
