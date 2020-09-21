extern crate raytrace_rs;

use std::process;
use std::time::SystemTime;

use raytrace_rs::xorshift::XorShift;
use raytrace_rs::ppm_gen::*;

fn main() {
    let width: usize = 100;
    let height: usize = 100;

    let mut rand = XorShift::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("failed to init RNG.").as_nanos() as u64);

    let mut buffer: Vec<Vec<RGB>> = Vec::with_capacity(height);
    buffer.resize(height, Vec::with_capacity(width));
    for x in &mut buffer { x.resize(width, RGB::new(0, 0, 0)); }

    for i in 0..height {
        for j in 0..width {
            buffer[i][j] = RGB::new((rand.next() % 256) as u8, (rand.next() % 256) as u8, (rand.next() % 256) as u8);
        }
    }

    if let Err(error) = generate_ppm(&buffer) {
        eprintln!("Error detected in generating ppm file.");
        eprintln!("Original error: {}", error);
        process::exit(1);
    }
}
