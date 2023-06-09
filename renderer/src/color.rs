use crate::vec3;
use crate::ray;

use vec3::Vec3 as Vec3;
use vec3::Vec3 as Color;
use ray::Ray as Ray;

pub const MAX_COLOR: f64 = 255.0;

pub fn write_color(pixel_color: Color) -> () {
    let ir: u8 = (pixel_color.x() * MAX_COLOR) as u8;
    let ig: u8 = (pixel_color.y() * MAX_COLOR) as u8;
    let ib: u8 = (pixel_color.z() * MAX_COLOR) as u8;
    println!("{} {} {}", ir, ig, ib)
}

pub fn ray_color(ray: &Ray) -> Color {
    let unit_direction: Vec3 = vec3::unit_vector(ray.direction);
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}