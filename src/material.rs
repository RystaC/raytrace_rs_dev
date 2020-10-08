use crate::sphere::*;
use crate::rgb::*;
use crate::ray::*;
use crate::vector::*;
use crate::xorshift::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool;
}

pub struct Lambertian {
    albedo: RGB,
}

impl Lambertian {
    pub fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool {
        let scatter_direction = record.normal + Vector3::randomized(rand);
        *scattered = Ray::new(record.position, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: RGB,
}

impl Metal {
    pub fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), record.normal);
        *scattered = Ray::new(record.position, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction, record.normal) > 0.0
    }
}