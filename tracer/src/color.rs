use crate::ray::Ray;
use crate::hittable::{HitRecord, HittableT};
use crate::vec3::{Vec3, Color, unit_vector};

pub const MAX_COLOR: f64 = 255.0;
pub const COLOR_WHITE: Color = Color::new_const(1.0, 1.0, 1.0);
pub const COLOR_SKY_BLUE: Color = Color::new_const(0.5, 0.7, 1.0);

pub fn write_color(pixel_color: Color) {
    let ir: u8 = (pixel_color.x() * MAX_COLOR) as u8;
    let ig: u8 = (pixel_color.y() * MAX_COLOR) as u8;
    let ib: u8 = (pixel_color.z() * MAX_COLOR) as u8;
    println!("{} {} {}", ir, ig, ib)
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