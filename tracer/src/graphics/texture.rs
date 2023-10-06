use std::sync::Arc;

use image::GenericImageView;

use crate::perlin::Perlin;
use crate::math::interval::Interval;
use crate::math::vec3::{Point3, Color};
use crate::rendering::color::{COLOR_BLACK, COLOR_WHITE};


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

// Image Texture
// =============
pub struct ImageTexture {
    image: image::DynamicImage
}

impl ImageTexture {
    pub fn new(path: &str) -> ImageTexture {
        ImageTexture { image: image::open(path).unwrap() }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u_clamped = Interval::new(0.0, 1.0).clamp(u);
        let v_clamped = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = u_clamped * (self.image.width() - 1) as f64;
        let j = v_clamped * (self.image.height() - 1) as f64;
        // println!("{} {}", i, j);
        let pixel = self.image.get_pixel(i as u32, j as u32);
        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

// Noise Texture
// ====================
#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture { noise: Perlin::default(), scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, point: &Point3) -> Color {
        // Interestingly, Peter's code sends a scaled point (self.scale*point)
        // to self.noise.turbulence(..), which generates a texture with a lot
        // of small turbulences - which does not look like the intended marble
        // he's aiming for in the book.
        // But, passing the point unscaled to the turbulance function AND
        // using the scaled point's Z component does result in the desired marble texture
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * point.z() + 10.0 * self.noise.turbulence(*point, 7)).sin())
    }
}
