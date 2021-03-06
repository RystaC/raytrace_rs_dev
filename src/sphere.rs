use std::sync::Arc;

use crate::vector::*;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub position: Vector3,
    pub normal: Vector3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front: bool,
}

impl HitRecord {
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new(0.0, 0.0, 0.0), material, t: 0.0, u: 0.0, v: 0.0, front: false }
    }
    #[inline(always)]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front = dot(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front { outward_normal } else { -outward_normal }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;

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
                let outward_normal = (record.position - self.center) / self.radius;
                record.set_face_normal(ray, outward_normal);
                record.material = self.material.clone();
                return true;
            }

            let temp = (-h_b + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.position = ray.at(record.t);
                let outward_normal = (record.position - self.center) / self.radius;
                record.set_face_normal(ray, outward_normal);
                get_sphere_uv(outward_normal, &mut record.u, &mut record.v);
                record.material = self.material.clone();
                return true;
            }

            else { false }
        }

        else { false }

    }
}

fn get_sphere_uv(p: Vector3, u: &mut f64, v: &mut f64) {
    let theta = -p.y.acos();
    let phi = -p.z.atan2(p.x) + std::f64::consts::PI;

    *u = phi / (2.0 * std::f64::consts::PI);
    *v = theta / std::f64::consts::PI;
}