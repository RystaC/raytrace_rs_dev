extern crate raytrace_rs;

use raytrace_rs::vector::*;

#[test]
fn vector_add() {
    let x = Vector3{ x: 3.0, y: 2.0, z: 1.0 };
    let y = Vector3{ x: 7.0, y: 3.0, z: 1.0 };

    assert_eq!(Vector3{ x: 10.0, y: 5.0, z: 2.0 }, x + y);

    let _z = x + y;
} 

#[test]
fn vector_sub() {
    let x = Vector3{ x: 3.0, y: 2.0, z: 1.0 };
    let y = Vector3{ x: 7.0, y: 3.0, z: 1.0 };

    assert_eq!(Vector3{ x: -4.0, y: -1.0, z: 0.0 }, x - y);

    let _z = x + y;
}

#[test]
fn mul_vf() {
    let x = Vector3{ x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(Vector3{ x: 2.0, y: 4.0, z: 6.0 }, x * 2.0);
}

#[test]
fn mul_fv() {
    let x = Vector3{ x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(Vector3{ x: 2.0, y: 4.0, z: 6.0 }, 2.0 * x);
}
