pub struct Encoder {
	pub encoders: Vec< fn( Vec<u8> ) -> Vec<u8> >
}

impl Encoder
{
	pub fn encode( &self, mut bufVec: Vec<u8> ) -> Vec<u8>
	{
		for subEncode in self.encoders.iter()
		{
			let thisEncode = subEncode.clone();
			bufVec = (thisEncode( bufVec ));
		}
		return bufVec;
	}
	
	pub fn clone( &self ) -> Encoder
	{
		return Encoder{ encoders: self.encoders.clone() };
	
	}
}
