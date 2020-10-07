use std::process;
use std::time::SystemTime;

use raytrace_rs::xorshift::XorShift;
use raytrace_rs::rgb::RGB;
use raytrace_rs::ppm_gen::generate_ppm;
use raytrace_rs::ray::Ray;
use raytrace_rs::vector::*;
use raytrace_rs::sphere::*;
use raytrace_rs::hittable_list::*;
use raytrace_rs::camera::*;

fn main() {
    let mut rand = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);

    let aspect = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = (width as f64 / aspect) as usize;
    let samples = 100;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut buffer: Vec<Vec<RGB>> = Vec::with_capacity(height);
    buffer.resize(height, Vec::with_capacity(width));
    for x in &mut buffer { x.resize(width, RGB::new(0.0, 0.0, 0.0)); }

    for i in 0..height {
        for j in 0..width {
            let mut pixel_color = RGB::new(0.0, 0.0, 0.0);
            for _s in 0..samples {
                let u = (j as f64 + (rand.next() as f64 / u64::MAX as f64)) / ((width - 1) as f64);
                let v = (i as f64 + (rand.next() as f64 / u64::MAX as f64)) / ((height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            buffer[i][j] = pixel_color;
        }
    }

    if let Err(error) = generate_ppm(&buffer, samples) {
        eprintln!("\nError detected in generating ppm file.");
        eprintln!("Original error: {}", error);
        process::exit(1);
    }
}

fn ray_color(ray: &Ray, world: &HittableList) -> RGB {
    let mut record = HitRecord::new();
    if world.hit(ray, 0.0, f64::MAX, &mut record) {
        RGB::from(0.5 * (record.normal + Vector3::new(1.0, 1.0, 1.0)))
    }

    else {
        let unit = ray.direction.normalize();
        let t = 0.5 * (unit.y + 1.0);
        RGB::from((1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0))
    }
}
