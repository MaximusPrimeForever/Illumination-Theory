use crate::ray::Ray;
use crate::interval::Interval;
use crate::math::vec3::{Vec3, Color};

// Colors
pub const COLOR_WHITE: Color = Color::new_const(1.0, 1.0, 1.0);
pub const COLOR_SKY_BLUE: Color = Color::new_const(0.7, 0.8, 1.0);
// pub const COLOR_RED: Color = Color::new_const(100.0, 0.0, 0.0);
pub const COLOR_BLACK: Color = Color::new_const(0.0, 0.0, 0.0);


pub fn rasterize_color(pixel_color: Color, samples_per_pixel: usize) -> image::Rgb<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    
    let intensity = Interval::new(0.0, 0.999);
    let r: u8 = (256.0 * intensity.clamp(r)) as u8;
    let g: u8 = (256.0 * intensity.clamp(g)) as u8;
    let b: u8 = (256.0 * intensity.clamp(b)) as u8;

    image::Rgb([r, g, b])
}

/// Return the color of the sky gradient when a ray hit it.
/// Blends smoothly between white, and light blue.
pub fn sky_color(ray: Ray) -> Color {
    let unit_direction: Vec3 = ray.direction.unit();
    let h = 0.5 * (unit_direction.y() + 1.0) * 1.0;

    (1.0 - h) * COLOR_WHITE + h * COLOR_SKY_BLUE
}
