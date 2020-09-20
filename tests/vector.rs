extern crate raytrace_rs;

use raytrace_rs::vector::*;

#[test]
fn vector_add() {
    assert_eq!(Vector3{ x: 10, y: 5, z: 2 }, Vector3{ x: 3, y: 2, z: 1 } + Vector3{ x: 7, y: 3, z: 1 });
} 

#[test]
fn vector_sub() {
    assert_eq!(Vector3{ x: -4, y: -1, z: 0 }, Vector3{ x: 3, y: 2, z: 1 } - Vector3{ x: 7, y: 3, z: 1 });
}