/// Misc math functions

use rand::random;
use crate::math::vec3::{Vec3, Point3};

pub fn get_random_point_in_unit_sphere() -> Point3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 { return p; }
    }
}

pub fn get_random_point_on_unit_sphere() -> Point3 {
    Vec3::random_range(-1.0, 1.0).unit()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_f64_in_range(-1.0, 1.0), random_f64_in_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 { return p }
    }
}

/// Generate a random number in a given half open range
/// [min, max)
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + random::<f64>() * (max - min)
}

