#![feature(test)]

extern crate raytrace_rs;
extern crate test;

use raytrace_rs::vector::*;

#[bench]
fn vector_add(b: &mut test::Bencher) {
    b.iter(|| Vector3{ x: 1.0, y: 2.0, z: 3.0 } + Vector3{ x: 4.0, y: 5.0, z: 6.0 });
}