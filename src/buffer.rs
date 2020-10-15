use std::sync::{Arc, Mutex};

use crate::rgb::*;

pub struct Buffer {}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Arc<Mutex<Vec<Vec<RGB>>>> {
        let mut buffer: Vec<Vec<RGB>> = Vec::with_capacity(height);
        buffer.resize(height, Vec::with_capacity(width));
        for x in &mut buffer { x.resize(width, RGB::new(0.0, 0.0, 0.0)); };
        Arc::from(Mutex::new(buffer))
    }
}