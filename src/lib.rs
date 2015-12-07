#[macro_export]
macro_rules! impl_elastic_array {
	($name: ident, $dummy: ident, $size: expr) => (
		#[doc(hidden)]
		enum $dummy <T> {
			Arr([T; $size]),
			Vec(Vec<T>)
		}

		impl <T> $dummy <T> {
			#[doc(hidden)]
			pub fn slice(&self) -> &[T] {
				match *self {
					$dummy::Arr(ref v) => v,
					$dummy::Vec(ref v) => v
				}
			}
		}

		pub struct $name<T> {
			raw: $dummy<T>,
			len: usize
		}

		impl <T> $name<T> where T: Copy {
			pub fn new() -> $name<T> {
				$name {
					raw: $dummy::Arr(unsafe { ::std::mem::uninitialized() }),
					len: 0
				}
			}

			pub fn push(&mut self, e: T) {
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

			pub fn pop(&mut self) -> Option<T> {
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

			pub fn append_slice(&mut self, elements: &[T]) {
				let len = self.len;
				self.insert_slice(len, elements)
			}

			pub fn to_vec(self) -> Vec<T> {
				match self.raw {
					$dummy::Arr(a) => {
						let mut vec = vec![];
						vec.reserve(self.len);
						unsafe {	
							::std::ptr::copy(a.as_ptr(), vec.as_mut_ptr(), self.len);
							vec.set_len(self.len);
						}
						vec
					}
					$dummy::Vec(v) => v
				}
			}

			pub fn insert_slice(&mut self, index: usize, elements: &[T]) {
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

		impl <T>::std::ops::Deref for $name<T> {
			type Target = [T];

			#[inline]
			fn deref(&self) -> &[T] {
				match self.raw {
					$dummy::Arr(ref a) => &a[..self.len],
					$dummy::Vec(ref v) => v
				}
			}
		}

		impl <T>::std::ops::DerefMut for $name<T> {
			#[inline]
			fn deref_mut(&mut self) -> &mut [T] {
				match self.raw {
					$dummy::Arr(ref mut a) => &mut a[..self.len],
					$dummy::Vec(ref mut v) => v
				}
			}
		}
	)
}

impl_elastic_array!(ElasticArray2, ElasticArray2Dummy, 2);
impl_elastic_array!(ElasticArray4, ElasticArray4Dummy, 4);
impl_elastic_array!(ElasticArray8, ElasticArray8Dummy, 8);
impl_elastic_array!(ElasticArray16, ElasticArray16Dummy, 16);
impl_elastic_array!(ElasticArray32, ElasticArray32Dummy, 32);
impl_elastic_array!(ElasticArray64, ElasticArray64Dummy, 64);
impl_elastic_array!(ElasticArray128, ElasticArray128Dummy, 128);
impl_elastic_array!(ElasticArray256, ElasticArray256Dummy, 256);
impl_elastic_array!(ElasticArray512, ElasticArray512Dummy, 512);
impl_elastic_array!(ElasticArray1024, ElasticArray1024Dummy, 1024);
impl_elastic_array!(ElasticArray2048, ElasticArray2048Dummy, 2048);

#[cfg(test)]
mod tests {

	type BytesShort = super::ElasticArray2<u8>;

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

	#[test]
	fn append_slice() {
		let mut bytes = BytesShort::new();
		bytes.push(1);
		bytes.append_slice(&[3, 4]);
		let r: &[u8] = &bytes;
		assert_eq!(r.len(), 3);
		assert_eq!(r, &[1, 3 ,4]);
	}
}


