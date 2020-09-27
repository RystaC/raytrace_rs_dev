use std::process;
//use std::time::SystemTime;

//use raytrace_rs::xorshift::XorShift;
use raytrace_rs::rgb::RGB;
use raytrace_rs::ppm_gen::generate_ppm;
use raytrace_rs::ray::Ray;
use raytrace_rs::vector::Vector3;

fn main() {
    let aspect = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = (width as f64 / aspect) as usize;

    let viewport_height = 2.0;
    let viewport_width = aspect * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    let mut buffer: Vec<Vec<RGB>> = Vec::with_capacity(height);
    buffer.resize(height, Vec::with_capacity(width));
    for x in &mut buffer { x.resize(width, RGB::new(0.0, 0.0, 0.0)); }

    for i in 0..height {
        for j in 0..width {
            let u = j as f64 / (width - 1) as f64;
            let v = i as f64 / (height - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            buffer[i][j] = ray_color(&r);
        }
    }

    if let Err(error) = generate_ppm(&buffer) {
        eprintln!("\nError detected in generating ppm file.");
        eprintln!("Original error: {}", error);
        process::exit(1);
    }
}

fn ray_color(ray: &Ray) -> RGB {
    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 0.5);
    RGB::from((1.0 - t) * Vector3::new(0.5, 0.7, 1.0) + t * Vector3::new(1.0, 1.0, 1.0))
}
