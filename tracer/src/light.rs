use crate::{Point3, Color};

pub struct Light {
    pub origin: Point3,
    pub color: Color,
    pub brightness: f64
}

impl Light {
    pub fn new(origin: Point3, color: Color, brightness: f64) -> Light {
        Light { origin, color, brightness }
    }
}