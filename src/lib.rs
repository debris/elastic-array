#[macro_export]
macro_rules! impl_elastic_array {
	($name: ident, $elem: ident, $size: expr) => (
		#[doc(hidden)]
		mod _inner {
			pub enum $name {
				Arr([$elem; $size]),
				Vec(Vec<$elem>)
			}

			impl $name {
				pub fn slice(&self) -> &[$elem] {
					match *self {
						$name::Arr(ref v) => v,
						$name::Vec(ref v) => v
					}
				}
			}
		}

		struct $name {
			raw: _inner::$name,
			len: usize
		}

		impl $name {
			pub fn new() -> $name {
				$name {
					raw: _inner::$name::Arr(unsafe { ::std::mem::uninitialized() }),
					len: 0
				}
			}

			pub fn push(&mut self, e: $elem) {
				match self.raw {
					_inner::$name::Arr(ref mut a) if self.len < a.len() => {
						unsafe {
							*a.get_unchecked_mut(self.len) = e;
						}
					},
					_inner::$name::Arr(_) => {
						let mut vec = vec![];
						vec.reserve(self.len + 1);

						unsafe {
							::std::ptr::copy(self.raw.slice().as_ptr(), vec.as_mut_ptr(), self.len);
							vec.set_len(self.len);
						}

						vec.push(e);
						self.raw = _inner::$name::Vec(vec);
					},
					_inner::$name::Vec(ref mut v) => v.push(e)
				}
				self.len += 1;
			}

			pub fn pop(&mut self) -> Option<$elem> {
				if self.len == 0 {
					return None;
				}

				self.len -= 1;
				match self.raw {
					_inner::$name::Arr(ref a) => Some(a[self.len]),
					_inner::$name::Vec(ref mut v) => v.pop()
				}
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
					_inner::$name::Arr(ref mut a) if len + elen <= a.len() => unsafe {
						let p = a.as_mut_ptr().offset(index as isize);
						let ep = elements.as_ptr();

						// shift everything by elen, to make space
						ptr::copy(p, p.offset(elen as isize), len - index);
						// write new elements
						ptr::copy(ep, p, elen);
					},
					// it deosn't, must be rewritten to vec
					_inner::$name::Arr(_) => unsafe {
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
						self.raw = _inner::$name::Vec(vec);
					},
					// just insert it in to vec
					_inner::$name::Vec(ref mut v) => unsafe {
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
					_inner::$name::Arr(ref a) => &a[..self.len],
					_inner::$name::Vec(ref v) => v
				}
			}
		}
	)
}

#[cfg(test)]
mod tests {
	impl_elastic_array!(BytesShort, u8, 2);
	
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
}


