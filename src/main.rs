use std::fs::File;
use std::io::{Write, BufWriter};

fn main() {
    let mut file = BufWriter::new(File::create("products/test.txt").expect("failed to create file."));

    file.write_all("Hello everyone.\n".as_bytes()).unwrap();
    file.write_all("This is test file.\n".as_bytes()).unwrap();
}
