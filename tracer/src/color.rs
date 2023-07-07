use crate::ray::Ray;
use crate::rtweekend::clamp;
use crate::hittable::{HitRecord, HittableT};
use crate::vec3::{Vec3, Color, unit_vector};

pub const MAX_COLOR: f64 = 255.0;
pub const COLOR_WHITE: Color = Color::new_const(1.0, 1.0, 1.0);
pub const COLOR_SKY_BLUE: Color = Color::new_const(0.5, 0.7, 1.0);

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8
    )
}

pub fn ray_color(ray: &Ray, world: &dyn HittableT) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + COLOR_WHITE);
    }

    let unit_direction: Vec3 = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * COLOR_WHITE + t * COLOR_SKY_BLUE
}