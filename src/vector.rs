use std::ops::*;

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
