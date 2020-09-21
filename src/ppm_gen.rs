use std::fs::File;
use std::io::{Write, BufWriter};
use std::error::Error;

#[derive(Clone, Copy, Debug)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        RGB { red, green, blue }
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

pub fn generate_ppm(buffer: &Vec<Vec<RGB>>) -> Result<(), Box<dyn Error>> {
    let mut file = BufWriter::new(File::create("products/test.ppm")?);

    file.write_all("P6\n".as_bytes())?;
    file.write_all(format!("{} {}\n", buffer[0].len(), buffer.len()).as_bytes())?;
    file.write_all("255\n".as_bytes())?;

    for i in 0..buffer.len() {
        for j in 0..buffer[0].len() {
            file.write_all(&buffer[i][j].as_bytes())?;
        }
    }

    Ok(())
}