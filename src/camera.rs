use crate::vector::*;
use crate::ray::*;
use crate::xorshift::*;

#[allow(dead_code)]
pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = cross(vup, w).normalize();
        let v = cross(w, u);
    
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
    
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, u: f64, v: f64, rand: &mut XorShift) -> Ray {
        let rd = self.lens_radius * Vector3::rand_in_unit_disk(rand);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }
}

#[inline(always)]
fn degrees_to_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}