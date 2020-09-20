extern crate raytrace_rs;

use std::fs::File;
use std::io::{Write, BufWriter};
use std::time::SystemTime;

use raytrace_rs::xorshift::XorShift;

#[derive(Clone, Copy, Debug, PartialEq)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        RGB { red, green, blue }
    }

    fn as_bytes(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

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


    let mut file = BufWriter::new(File::create("products/test.ppm").expect("failed to create file."));

    file.write_all("P6\n".as_bytes()).unwrap();
    file.write_all(format!("{} {}\n", width, height).as_bytes()).unwrap();
    file.write_all("255\n".as_bytes()).unwrap();

    for i in 0..height {
        for j in 0..width {
            file.write_all(&buffer[i][j].as_bytes()).unwrap();
        }
    }

}
