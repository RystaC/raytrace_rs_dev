use super::vector::Vector3;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct RGB {
    red: f64,
    green: f64,
    blue: f64,
}

impl RGB {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        RGB { red, green, blue }
    }

    pub fn from(v: Vector3) -> Self {
        RGB { red: v.x, green: v.y, blue: v.z }
    }

    pub fn as_bytes(&self, sample_p_pixel: i32) -> [u8; 3] {
        let mut red = self.red;
        let mut green = self.green;
        let mut blue = self.blue;

        let scale = 1.0 / sample_p_pixel as f64;

        red *= scale;
        green *= scale;
        blue *= scale;

        [gamma_correct(clamp(red)), gamma_correct(clamp(green)), gamma_correct(clamp(blue))]
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
    //(x.powf(1.0 / 2.2) * 255.0 + 0.5) as u8
    (x * 255.0 + 0.5) as u8
}

impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}
