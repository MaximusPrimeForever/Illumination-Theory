use crate::vec3;
use vec3::Vec3 as Color;

pub const MAX_COLOR: f64 = 255.0;

pub fn write_color(pixel_color: Color) -> () {
    let ir: u8 = (pixel_color.x() * MAX_COLOR) as u8;
    let ig: u8 = (pixel_color.y() * MAX_COLOR) as u8;
    let ib: u8 = (pixel_color.z() * MAX_COLOR) as u8;
    println!("{} {} {}", ir, ig, ib)
}