
use crate::{vec3::{Vec3, Point3}, rtweekend::random_f64_range};

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
        let p = Vec3::new(random_f64_range(-1.0, 1.0), random_f64_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 { return p }
    }
}