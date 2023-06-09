use crate::ray::Ray;
use crate::buffer::Pixel;
use crate::rtweekend::clamp;
use crate::hittable::{HittableT};

use crate::vec3::{
    Vec3,
    Color
};

pub const MAX_COLOR: f64 = 255.0;
pub const COLOR_WHITE: Color = Color::new_const(1.0, 1.0, 1.0);
pub const COLOR_SKY_BLUE: Color = Color::new_const(0.5, 0.7, 1.0);
pub const COLOR_BLACK: Color = Color::new_const(0.0, 0.0, 0.0);
pub const T_MIN_TOLERANCE: f64 = 0.001;


pub fn write_color(pixel_color: Color, samples_per_pixel: u32) -> Pixel {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    
    let r: u8 = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let g: u8 = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let b: u8 = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    Pixel{r, g ,b}
}

/// Return the color of the sky gradient when a ray hit it.
/// Blends smoothly between white, and light blue.
pub fn sky_color(ray: &Ray) -> Color {
    let unit_direction: Vec3 = ray.direction.unit();
    let h = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - h) * COLOR_WHITE + h * COLOR_SKY_BLUE
}

/// Given a ray and a world, return the ray's color.
/// 
/// If the ray hit nothing, return the sky color.
pub fn ray_color(ray: &Ray, world: &dyn HittableT, depth: i32) -> Color {
    if depth <= 0 { return COLOR_BLACK; }
    
    match world.hit(ray, T_MIN_TOLERANCE, f64::INFINITY) {
        Some(rec) => {
            match rec.material.scatter(&ray, &rec) {
                Some((attenuation, scattered)) => {
                    attenuation * ray_color(&scattered, world, depth - 1)
                }
                None => { COLOR_BLACK }
            }
        }
        None => { sky_color(ray) }
    }
}
