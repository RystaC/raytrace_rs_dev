use std::process;
use std::time::{Instant, SystemTime};
use std::sync::Arc;
use std::thread;
use raytrace_rs::xorshift::*;
use raytrace_rs::rgb::*;
use raytrace_rs::ppm_gen::*;
use raytrace_rs::ray::*;
use raytrace_rs::vector::*;
use raytrace_rs::sphere::*;
use raytrace_rs::hittable_list::*;
use raytrace_rs::camera::*;
use raytrace_rs::material::*;
use raytrace_rs::buffer::*;

fn main() {
    // Config for parallelism
    let division = 4;

    // RNG
    let mut rand = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);

    // Image
    let aspect = 16.0 / 9.0;
    let width: usize = 512;
    let height: usize = (width as f64 / aspect) as usize;
    let samples = 128;
    let max_depth = 50;

    // World
    let world = Arc::from(random_scene(&mut rand));

    // Camera
    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Arc::from(Camera::new(lookfrom, lookat, vup, 20.0, aspect, aperture, dist_to_focus));

    // Buffer
    let buffer = Buffer::new(width, height);

    // Trace
    eprintln!("Generate rays:");
    let start = Instant::now();

    // Data handler for each threads
    let mut buffer_handlers = vec![];
    let mut world_handlers = vec![];
    let mut camera_handlers = vec![];

    for _ in 0..division {
        buffer_handlers.push(buffer.clone());
        world_handlers.push(world.clone());
        camera_handlers.push(camera.clone());
    }

    // Thread handler
    let mut thread_handlers = vec![];

    // Parallelism
    for (((n, buffer_handler), world_handler), camera_handler) in (0..division)
                                                                    .zip(buffer_handlers)
                                                                    .zip(world_handlers)
                                                                    .zip(camera_handlers)
    {
        let handle = thread::spawn(move || {
            let mut rand_t = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64);
            let proc_div = height / division;
            for i in (proc_div * n)..(proc_div * (n + 1)) {
                for j in 0..width {
                    let mut pixel_color = RGB::new(0.0, 0.0, 0.0);
                    for _s in 0..samples {
                        let u = (j as f64 + rand_t.next_normalize()) / (width - 1) as f64;
                        let v = (i as f64 + rand_t.next_normalize()) / (height - 1) as f64;
                        let r = camera_handler.get_ray(u, v, &mut rand_t);
                        pixel_color += ray_color(&r, &world_handler, max_depth, &mut rand_t);
                    }
                    buffer_handler.lock().unwrap()[i][j] = pixel_color;
                }
                eprint!("\r    Progress(thread{}): {:.1}% ({}/{}) done.", n, (((i + 1) - proc_div * n) as f64 / proc_div as f64) * 100.0, (i + 1) - proc_div * n, proc_div);
            }
        });
        thread_handlers.push(handle);
    }

    for thread_handle in thread_handlers {
        thread_handle.join().unwrap();
    }

    let end = start.elapsed();
    eprintln!("\n\nFinished. ({}.{:03} seconds elapsed)", end.as_secs(), end.subsec_nanos() / 1000000);

    // Generate Image
    if let Err(error) = generate_ppm(&buffer.clone().lock().unwrap(), samples) {
        eprintln!("\nError detected in generating ppm file.");
        eprintln!("Original error: {}", error);
        process::exit(1);
    }
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32, rand: &mut XorShift) -> RGB {
    let mut record = HitRecord::new(Arc::from(Lambertian::new(RGB::new(0.0, 0.0, 0.0))));

    if depth <= 0 { return RGB::new(0.0, 0.0, 0.0); }

    if world.hit(ray, 0.0001, f64::MAX, &mut record) {
        let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut attenuation = RGB::new(0.0, 0.0, 0.0);
        if record.material.scatter(ray, &record, &mut attenuation, &mut scattered, rand) {
            attenuation * ray_color(&scattered, world, depth - 1, rand)
        }
        else { RGB::new(0.0, 0.0, 0.0) }
    }

    else {
        let unit = ray.direction.normalize();
        let t = 0.5 * (unit.y + 1.0);
        RGB::from((1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0))
    }
}

fn random_scene(rand: &mut XorShift) -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::from(Lambertian::new(RGB::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand.next_normalize();
            let center = Vector3::new(a as f64 + 0.9 * rand.next_normalize(), 0.2, b as f64 + 0.9 * rand.next_normalize());

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = RGB::new(rand.next_normalize(), rand.next_normalize(), rand.next_normalize());
                    let sphere_material = Arc::from(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    let albedo = RGB::new(rand.next_normalize(), rand.next_normalize(), rand.next_normalize());
                    let fuzz = rand.next_bounded(0.0, 0.5);
                    let sphere_material = Arc::from(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else {
                    let sphere_material = Arc::from(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::from(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::from(Lambertian::new(RGB::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::from(Metal::new(RGB::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}
