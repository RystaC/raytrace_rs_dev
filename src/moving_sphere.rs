use std::sync::Arc;

use crate::sphere::*;
use crate::vector::*;
use crate::material::*;
use crate::ray::*;

pub struct MovingSphere {
    pub center0: Vector3,
    pub center1: Vector3,
    pub t0: f64,
    pub t1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(center0: Vector3, center1: Vector3, t0: f64, t1: f64, radius: f64, material: Arc<dyn Material>) -> Self {
        Self { center0, center1, t0, t1, radius, material }
    }

    pub fn center(&self, time: f64) -> Vector3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center(ray.time);

        let a = dot(ray.direction, ray.direction);
        let h_b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius.powf(2.0);
    
        let discriminant = h_b * h_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-h_b - root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.position = ray.at(record.t);
                let outward_normal = (record.position - self.center(ray.time)) / self.radius;
                record.set_face_normal(ray, outward_normal);
                record.material = self.material.clone();
                return true;
            }

            let temp = (-h_b + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.position = ray.at(record.t);
                let outward_normal = (record.position - self.center(ray.time)) / self.radius;
                record.set_face_normal(ray, outward_normal);
                record.material = self.material.clone();
                return true;
            }

            else { false }
        }

        else { false }

    }
}