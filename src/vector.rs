use std::ops::*;
use std::f64::consts::*;

use crate::xorshift::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn randomized(rand: &mut XorShift) -> Self {
        let a = rand.next_bounded(0.0, 2.0 * PI);
        let z = rand.next_bounded(-1.0, 1.0);
        let r = f64::sqrt(1.0 - z * z);
        Vector3::new(r * f64::cos(a), r * f64::sin(a), z)
    }

    pub fn norm(&self) -> f64 {
        f64::sqrt(f64::powf(self.x, 2.0) + f64::powf(self.y, 2.0) + f64::powf(self.z, 2.0))
    }

    pub fn normalize(&self) -> Self {
        *self / self.norm()
    }
}

pub fn dot(lhs: Vector3, rhs: Vector3) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn cross(lhs: Vector3, rhs: Vector3) -> Vector3 {
    Vector3 { x: lhs.y * rhs.z - lhs.z * rhs.y, y: lhs.z * rhs.x - lhs.x * rhs.z, z: lhs.x * rhs.y - lhs.y * rhs.x }
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vector3, n: Vector3, eoe: f64) -> Vector3 {
    let cos_theta = dot(-uv, n);
    let r_perp = eoe * (uv + cos_theta * n);
    let r_parallel = -f64::sqrt(f64::abs(1.0 - dot(r_perp, r_perp))) * n;
    r_perp + r_parallel
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}
