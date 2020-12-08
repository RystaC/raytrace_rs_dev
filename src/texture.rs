use std::sync::Arc;

use crate::rgb::*;
use crate::vector::*;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vector3) -> RGB;
}

pub struct SolidColor {
    color_value: RGB,
}

impl SolidColor {
    pub fn new(c: RGB) -> Self {
        Self { color_value: c }
    }
}

impl Texture for SolidColor {
    #[allow(unused_variables)]
    fn value(&self, u: f64, v: f64, p: Vector3) -> RGB {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: RGB, c2: RGB) -> Self {
        Self { odd: Arc::from(SolidColor::new(c1)), even: Arc::from(SolidColor::new(c2)) }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vector3) -> RGB {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 { self.odd.value(u, v, p) }
        else { self.even.value(u, v, p) }
    }
}