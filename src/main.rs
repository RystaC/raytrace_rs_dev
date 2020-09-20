use std::fs::File;
use std::io::{Write, BufWriter};

#[derive(Clone, Copy, Debug, PartialEq)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let width: usize = 100;
    let height: usize = 100;

    let mut buffer: Vec<Vec<RGB>> = Vec::with_capacity(height);
    buffer.resize(height, Vec::with_capacity(width));
    for x in &mut buffer { x.resize(width, RGB{ red: 0, green: 255, blue: 0 }); }


    let mut file = BufWriter::new(File::create("products/test.ppm").expect("failed to create file."));

    file.write_all("P6\n".as_bytes()).unwrap();
    file.write_all(format!("{} {}\n", width, height).as_bytes()).unwrap();
    file.write_all("255\n".as_bytes()).unwrap();

    for i in 0..height {
        for j in 0..width {
            file.write_all(&[buffer[i][j].red, buffer[i][j].green, buffer[i][j].blue]).unwrap();
        }
    }

}
