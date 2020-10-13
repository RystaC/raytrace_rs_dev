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
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: RGB, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), record.normal);
        *scattered = Ray::new(record.position, reflected + self.fuzz * rand_unit_sphere(rand));
        *attenuation = self.albedo;
        dot(scattered.direction, record.normal) > 0.0
    }
}

fn rand_unit_sphere(rand: &mut XorShift) -> Vector3 {
    loop {
        let p = Vector3::new(rand.next_bounded(-1.0, 1.0), rand.next_bounded(-1.0, 1.0), rand.next_bounded(-1.0, 1.0));
        if dot(p, p) < 1.0 { return p; }
    }
}