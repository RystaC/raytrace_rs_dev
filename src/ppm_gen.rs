use std::fs::File;
use std::io::{Write, BufWriter};
use std::error::Error;

#[derive(Clone, Copy, Debug)]
pub struct RGB {
    red: f64,
    green: f64,
    blue: f64,
}

impl RGB {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        RGB { red: clamp(red), green: clamp(green), blue: clamp(blue) }
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        [gamma_correct(self.red), gamma_correct(self.green), gamma_correct(self.blue)]
    }
}

#[inline(always)]
fn clamp(x: f64) -> f64 {
    if x < 0.0 { 0.0 }
    else if x > 1.0 { 1.0 }
    else { x }
}

#[inline(always)]
fn gamma_correct(x: f64) -> u8 {
    (x.powf(1.0 / 2.2) * 255.0 + 0.5) as u8
}

pub fn generate_ppm(buffer: &Vec<Vec<RGB>>) -> Result<(), Box<dyn Error>> {
    let width = buffer[0].len();
    let height = buffer.len();

    let mut file = BufWriter::new(File::create("products/test.ppm")?);

    file.write_all("P6\n".as_bytes())?;
    file.write_all(format!("{} {}\n", width, height).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    for i in 0..height {
        for j in 0..width {
            file.write_all(&buffer[i][j].as_bytes())?;
        }
    }

    Ok(())
}