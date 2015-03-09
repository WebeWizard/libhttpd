use std::sync::mpsc::{Sender,Receiver};

pub struct Encoder {
	pub name: &'static str,
	pub weight: u8, // default will be 100u8, heavier weights will take preference over lighter weights
	pub encode: fn( Receiver<Vec<u8>>, Sender<Vec<u8>>)
}

impl Clone for Encoder {
	fn clone(&self) -> Encoder {
		return Encoder {
			name : self.name.clone(),
			weight: self.weight.clone(),
			encode : self.encode.clone()
		}
	}
}
