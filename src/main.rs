extern crate raytrace_rs;

use std::process;
//use std::time::SystemTime;

//use raytrace_rs::xorshift::XorShift;
use raytrace_rs::ppm_gen::*;

fn main() {
    let width: usize = 256;
    let height: usize = 256;

    //let mut rand = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("failed to init RNG.").as_nanos() as u64);

    let mut buffer: Vec<Vec<RGB>> = Vec::with_capacity(height);
    buffer.resize(height, Vec::with_capacity(width));
    for x in &mut buffer { x.resize(width, RGB::new(0.0, 0.0, 0.0)); }

    for i in 0..height {
        for j in 0..width {
            //buffer[i][j] = RGB::new((rand.next() % 256) as u8, (rand.next() % 256) as u8, (rand.next() % 256) as u8);
            buffer[i][j] = RGB::new(i as f64 / (width - 1) as f64, j as f64 / (height - 1) as f64, 0.25);
        }
    }

    if let Err(error) = generate_ppm(&buffer) {
        eprintln!("\nError detected in generating ppm file.");
        eprintln!("Original error: {}", error);
        process::exit(1);
    }
}
