use super::vector::Vector3;

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

    pub fn as_bytes(&self) -> [u8; 3] {
        [gamma_correct(clamp(self.red)), gamma_correct(clamp(self.green)), gamma_correct(clamp(self.blue))]
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
