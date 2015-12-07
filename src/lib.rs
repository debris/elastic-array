#[macro_export]
macro_rules! impl_elastic_array {
	($name: ident, $dummy: ident, $elem: ident, $size: expr) => (
		#[doc(hidden)]
		enum $dummy {
			Arr([$elem; $size]),
			Vec(Vec<$elem>)
		}

		impl $dummy {
			#[doc(hidden)]
			pub fn slice(&self) -> &[$elem] {
				match *self {
					$dummy::Arr(ref v) => v,
					$dummy::Vec(ref v) => v
				}
			}
		}

		struct $name {
			raw: $dummy,
			len: usize
		}

		impl $name {
			pub fn new() -> $name {
				$name {
					raw: $dummy::Arr(unsafe { ::std::mem::uninitialized() }),
					len: 0
				}
			}

			pub fn push(&mut self, e: $elem) {
				match self.raw {
					$dummy::Arr(ref mut a) if self.len < a.len() => {
						unsafe {
							*a.get_unchecked_mut(self.len) = e;
						}
					},
					$dummy::Arr(_) => {
						let mut vec = vec![];
						vec.reserve(self.len + 1);

						unsafe {
							::std::ptr::copy(self.raw.slice().as_ptr(), vec.as_mut_ptr(), self.len);
							vec.set_len(self.len);
						}

						vec.push(e);
						self.raw = $dummy::Vec(vec);
					},
					$dummy::Vec(ref mut v) => v.push(e)
				}
				self.len += 1;
			}

			pub fn pop(&mut self) -> Option<$elem> {
				if self.len == 0 {
					return None;
				}

				self.len -= 1;
				match self.raw {
					$dummy::Arr(ref a) => Some(a[self.len]),
					$dummy::Vec(ref mut v) => v.pop()
				}
			}

			pub fn clear(&mut self) {
				self.raw = $dummy::Arr(unsafe { ::std::mem::uninitialized() });
				self.len = 0;
			}

			pub fn insert_slice(&mut self, index: usize, elements: &[$elem]) {
				use std::ptr;

				let elen = elements.len();

				if elen == 0 {
					return;
				}
				
				let len = self.len;
				assert!(index <= len);

				match self.raw {
					// it fits in array
					$dummy::Arr(ref mut a) if len + elen <= a.len() => unsafe {
						let p = a.as_mut_ptr().offset(index as isize);
						let ep = elements.as_ptr();

						// shift everything by elen, to make space
						ptr::copy(p, p.offset(elen as isize), len - index);
						// write new elements
						ptr::copy(ep, p, elen);
					},
					// it deosn't, must be rewritten to vec
					$dummy::Arr(_) => unsafe {
						let mut vec = vec![];
						vec.reserve(self.len + elen);
						{
							let p = vec.as_mut_ptr();
							let ob = self.raw.slice().as_ptr();
							let ep = elements.as_ptr();
							let oe = ob.offset(index as isize);
							
							// copy begining of an array
							ptr::copy(ob, p, index);

							// copy new elements
							ptr::copy(ep, p.offset(index as isize), elen);

							// copy end of an array	
							ptr::copy(oe, p.offset((index + elen) as isize), len - index);
						}
						vec.set_len(self.len + elen);
						self.raw = $dummy::Vec(vec);
					},
					// just insert it in to vec
					$dummy::Vec(ref mut v) => unsafe {
						v.reserve(elen);

						let p = v.as_mut_ptr().offset(index as isize);
						let ep = elements.as_ptr();

						// shift everything by elen, to make space
						ptr::copy(p, p.offset(elen as isize), len - index);
						// write new elements
						ptr::copy(ep, p, elen);

						v.set_len(self.len + elen);
					}
				}
				self.len += elen;
			}
		}

		impl ::std::ops::Deref for $name {
			type Target = [$elem];

			#[inline]
			fn deref(&self) -> &[$elem] {
				match self.raw {
					$dummy::Arr(ref a) => &a[..self.len],
					$dummy::Vec(ref v) => v
				}
			}
		}

		impl ::std::ops::DerefMut for $name {
			#[inline]
			fn deref_mut(&mut self) -> &mut [$elem] {
				match self.raw {
					$dummy::Arr(ref mut a) => &mut a[..self.len],
					$dummy::Vec(ref mut v) => v
				}
			}
		}
	)
}



#[cfg(test)]
mod tests {

	impl_elastic_array!(BytesShort, BytesShortDummy, u8, 2);
	
	#[test]
	fn ret_struct() {
	}

	#[test]
	fn it_works() {
		let mut bytes = BytesShort::new();
		assert_eq!(bytes.len(), 0);
		bytes.push(1);
		assert_eq!(bytes.len(), 1);
		assert_eq!(bytes[0], 1);
		bytes.push(2);
		assert_eq!(bytes[1], 2);
		assert_eq!(bytes.len(), 2);
		bytes.push(3);
		assert_eq!(bytes[2], 3);
		assert_eq!(bytes.len(), 3);
		assert_eq!(bytes.pop(), Some(3));
		assert_eq!(bytes.len(), 2);
		assert_eq!(bytes.pop(), Some(2));
		assert_eq!(bytes.pop(), Some(1));
		assert_eq!(bytes.pop(), None);
	}

	#[test]
	fn test_insert_slice() {
		let mut bytes = BytesShort::new();
		bytes.push(1);
		bytes.push(2);
		bytes.insert_slice(1, &[3, 4]);
		assert_eq!(bytes.len(), 4);
		let r: &[u8] = &bytes;
		assert_eq!(r, &[1, 3, 4, 2]);
	}
}


