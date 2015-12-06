pub struct BytesArr1024 {
	arr: [u8; 1024]
}

impl BytesArr1024 {
	pub fn new() -> BytesArr1024 {
		BytesArr1024 {
			arr: [0; 1024]
		}
	}

	pub fn push(&mut self, e: u8) {
		self.arr[0] = e;
	}
}

pub struct BytesVec1024 {
	vec: Vec<u8>
}

impl BytesVec1024 {
	pub fn new() -> BytesVec1024 {
		BytesVec1024 {
			vec: vec![]
		}
	}

	pub fn push(&mut self, e: u8) {
		self.vec.push(e);
	}
}
