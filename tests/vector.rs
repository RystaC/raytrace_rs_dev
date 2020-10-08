use raytrace_rs::vector::*;

#[test]
fn test_vector_add() {
    let x = Vector3::new(3.0, 2.0, 1.0);
    let y = Vector3::new(7.0, 3.0, 1.0);

    assert_eq!(x + y, Vector3::new(10.0, 5.0, 2.0));
} 

#[test]
fn test_vector_sub() {
    let x = Vector3::new(3.0, 2.0, 1.0);
    let y = Vector3::new(7.0, 3.0, 1.0);

    assert_eq!(x - y, Vector3::new(-4.0, -1.0, 0.0));
}

#[test]
fn test_vector_mul_vf() {
    let x = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(x * 2.0, Vector3::new(2.0, 4.0, 6.0));
}

#[test]
fn test_vector_mul_fv() {
    let x = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(2.0 * x, Vector3::new(2.0, 4.0, 6.0));
}

#[test]
fn test_vector_dot() {
    let x = Vector3::new(1.0, -2.0, 3.0);
    let y = Vector3::new(3.0, 2.0, -1.0);
    assert_eq!(dot(x, y), -4.0)
}

#[test]
fn test_vector_cross() {
    let x = Vector3::new(1.0, 2.0, 3.0);
    let y = Vector3::new(3.0, 4.0, 5.0);
    assert_eq!(cross(x, y), Vector3::new(-2.0, 4.0, -2.0));
}
