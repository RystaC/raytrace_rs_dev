use std::fs::File;
use std::io::{Write, BufWriter};
use std::error::Error;

use crate::rgb::RGB;

pub fn generate_ppm(buffer: &Vec<Vec<RGB>>, samples: i32) -> Result<(), Box<dyn Error>> {
    let width = buffer[0].len();
    let height = buffer.len();

    let mut file = BufWriter::new(File::create("products/test.ppm")?);

    file.write_all("P6\n".as_bytes())?;
    file.write_all(format!("{} {}\n", width, height).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    for i in (0..height).rev() {
        for j in 0..width {
            file.write_all(&buffer[i][j].as_bytes(samples))?;
        }
    }

    Ok(())
}