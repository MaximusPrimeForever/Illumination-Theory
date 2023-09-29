use std::sync::Arc;

use crate::vec3::{Point3, Color};
use crate::color::{COLOR_BLACK, COLOR_WHITE};


pub trait Texture {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color;
}

pub type TextureSync = dyn Texture + Send + Sync;


// Solid Color
// ===========

#[derive(Default)]
pub struct SolidColorTexture {
    value: Color
}

impl SolidColorTexture {
    pub fn new(color: Color) -> Self {
        SolidColorTexture { value: color }
    }

    pub fn new_rgb(red: f64, blue: f64, green: f64) -> Self {
        SolidColorTexture { value: Color::new(red, blue, green) }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Color {
        return self.value;
    }
}


// Checkered
// =========
#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<TextureSync>,
    odd: Arc<TextureSync>
}

impl Default for CheckerTexture {
    fn default() -> Self {
        CheckerTexture { 
            inv_scale: 1.0, 
            even: Arc::new(SolidColorTexture::new(COLOR_BLACK.clone())),
            odd: Arc::new(SolidColorTexture::new(COLOR_WHITE.clone()))
        }
    }
}

impl CheckerTexture {
    pub fn new(scale: f64, _even: Arc<TextureSync>, _odd: Arc<TextureSync>) -> Self {
        CheckerTexture { inv_scale: 1.0 / scale, even: _even, odd: _odd }
    }

    pub fn new_color(_scale: f64, even: Color, odd: Color) -> Self {
        CheckerTexture { 
            inv_scale: 1.0 / _scale,
            even: Arc::new(SolidColorTexture::new(even)),
            odd: Arc::new(SolidColorTexture::new(odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color {
        let x_int = (self.inv_scale * (point.x())).floor() as i64;
        let y_int = (self.inv_scale * (point.y())).floor() as i64;
        let z_int = (self.inv_scale * (point.z())).floor() as i64;

        if (x_int + y_int + z_int) % 2 == 0 {
            return self.even.value(u, v, point);
        } else {
            return self.odd.value(u, v, point);
        }

    }
}