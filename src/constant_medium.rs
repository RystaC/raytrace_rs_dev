use std::sync::Arc;
use std::time::SystemTime;

use super::sphere::*;
use super::material::*;
use super::texture::*;
use super::rgb::*;
use super::ray::*;
use super::xorshift::*;
use super::vector::*;

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, c: RGB) -> Self {
        Self { boundary: b, phase_function: Arc::new(Isotropic::new(c)), neg_inv_density: -1.0 / d }
    }

    pub fn from(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        Self { boundary: b, phase_function: Arc::new(Isotropic::from(a)), neg_inv_density: -1.0 / d }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut rand = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);

        let mut rec1 = HitRecord::new(Arc::from(Lambertian::new(RGB::new(0.0, 0.0, 0.0))));
        let mut rec2 = HitRecord::new(Arc::from(Lambertian::new(RGB::new(0.0, 0.0, 0.0))));

        if !self.boundary.hit(ray, -f64::MAX, f64::MAX, &mut rec1) { return false; };
        if !self.boundary.hit(ray, rec1.t+0.0001, f64::MAX, &mut rec2) { return false; };

        if rec1.t < t_min { rec1.t = t_min; };
        if rec2.t > t_max { rec2.t = t_max; };

        if rec1.t >= rec2.t { return false; };

        if rec1.t < 0.0 { rec1.t = 0.0; };

        let ray_length = ray.direction.norm();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(rand.next_normalize());

        if hit_distance > distance_inside_boundary { return false; };

        record.t = rec1.t + hit_distance / ray_length;
        record.position = ray.at(record.t);
        record.normal = Vector3::new(1.0, 0.0, 0.0);
        record.front = true;
        record.material = self.phase_function.clone();

        true
    }
}