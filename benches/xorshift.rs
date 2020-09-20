#![feature(test)]

extern crate raytrace_rs;
extern crate test;

use std::time::SystemTime;
use raytrace_rs::xorshift::*;

#[bench]
fn bench_xorshift(b: &mut test::Bencher) {
    let mut rand = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);
    b.iter(|| rand.next() as f64 / u64::MAX as f64);
}