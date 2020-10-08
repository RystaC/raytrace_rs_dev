use crate::sphere::*;
use super::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp = *record;
        let mut hit_any = false;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest, &mut temp) {
                hit_any = true;
                closest = temp.t;
                *record = temp;
            }
        }

        hit_any
    }
}