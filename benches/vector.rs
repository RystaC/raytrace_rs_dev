#![feature(test)]

extern crate raytrace_rs;
extern crate test;

use raytrace_rs::vector::*;

#[bench]
fn vector_add(b: &mut test::Bencher) {
    b.iter(|| Vector3{ x: 1, y: 2, z: 3 } + Vector3{ x: 4, y: 5, z: 6 });
}