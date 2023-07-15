use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vertical_fov: f64, aspect_ratio: f64) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio *  viewport_height;

        let look_at_vec = (look_from - look_at).unit();
        let horizontal_vec = vup.cross(look_at_vec).unit();
        let vertical_vec = look_at_vec.cross(horizontal_vec);

        let origin = look_from;
        let vertical = viewport_height * vertical_vec;
        let horizontal = viewport_width * horizontal_vec;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - look_at_vec;

        Camera { 
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction =
            self.lower_left_corner
            + s * self.horizontal
            + t * self.vertical
            - self.origin;

        Ray::new(
            self.origin, 
            direction
        )
    }
}