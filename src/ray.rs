use crate::vector::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3, time: f64) -> Self {
        Self { origin, direction, time }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}