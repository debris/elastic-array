pub trait ElasticArray: Sized {
	type Elem;

	fn new(e: Self::Elem) -> Self;
	fn new_uninitialized() -> Self;
	fn push(self, e: Self::Elem) -> Self;
}

macro_rules! impl_elastic_arr {
	($name: ident, $elem: ident, $size: expr) => (
		pub enum $name {
			Arr([$elem; $size], usize),
			Vec(Vec<$elem>)
		}

		impl ElasticArray for $name {
			type Elem = $elem;

			fn new(e: $elem) -> $name {
				$name::Arr([e; $size], 0)
			}

			fn new_uninitialized() -> $name {
				$name::Arr(unsafe { ::std::mem::uninitialized() }, 0)
			}

			fn push(mut self, e: $elem) -> $name {
				match &mut self {
					&mut $name::Vec(ref mut v) => {
						v.push(e);
					},
					&mut $name::Arr(ref mut v, ref mut len) if v.len() != *len => {
						v[*len] = e;
						*len += 1;
					},
					&mut $name::Arr(ref v, len) => {
						use std::ptr;
						
						let mut res = vec![];
						res.reserve(len * 2);
						unsafe {
							ptr::copy(v.as_ptr(), res.as_mut_ptr(), len);
							res.set_len(len);
						}
						res.push(e);
						return $name::Vec(res);
					}
				};
				self
			}
		}

		impl ::std::ops::Deref for $name {
			type Target = [u8];

			#[inline]
			fn deref(&self) -> &[u8] {
				match self {
					&$name::Vec(ref v) => v,
					&$name::Arr(ref v, _) => v
				}
			}
		}
	)
}





#[cfg(test)]
mod tests {
	use ElasticArray;

	impl_elastic_arr!(Bytes, u8, 2048);
	impl_elastic_arr!(Bytes2, u8, 2048);

	#[test]
	fn it_works() {
		
	}
}


