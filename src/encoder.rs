use std::sync::mpsc::{Sender,Receiver};

pub struct Encoder {
	pub name: &'static str,
	pub encode: fn( Receiver<Vec<u8>>, Sender<Vec<u8>>)
}

impl Clone for Encoder {
	fn clone(&self) -> Encoder {
		return Encoder {
			name : self.name.clone(),
			encode : self.encode.clone()
		}
	}
}
