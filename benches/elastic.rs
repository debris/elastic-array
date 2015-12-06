//! should be started with:
//! //! ```bash
//! //! multirust run nightly cargo bench
//! //! ```

#![feature(test)]
extern crate test;
extern crate rand;

#[macro_use]
extern crate elastic_array;
extern crate to_compare;

use test::Bencher;
use rand::random;

use to_compare::*;

const LEN: usize = 2048;

fn gen_data() -> [u8; LEN] {
	let mut arr = [0u8; LEN];
	for i in 0..LEN {
		arr[i] = random::<u8>();
	}
	arr
}

impl_elastic_array!(BytesShort, u8, 1024);

#[bench]
fn bench_elastic_array(b: &mut Bencher) {
	let data = gen_data();
	b.iter(|| {
		let f = test::black_box(0);
		let n = test::black_box(LEN);
		let mut bytes = BytesShort::new();
		for i in f..n {
			bytes.push(data[i]);
		}
	});
}

#[bench]
fn bench_vector(b: &mut Bencher) {
	let data = gen_data();
	b.iter(|| {
		let f = test::black_box(0);
		let n = test::black_box(LEN);
		let mut v = BytesVec1024::new();
		for i in f..n {
			v.push(data[i]);
		}
	});
}

#[bench]
fn bench_arr(b: &mut Bencher) {
	let data = gen_data();
	b.iter(|| {
		let f = test::black_box(0);
		let n = test::black_box(LEN);
		let mut arr = BytesArr1024::new();
		for i in f..n {
			arr.push(data[i]);
		}
	});
}
