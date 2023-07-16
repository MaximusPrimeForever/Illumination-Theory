use crate::geometry::random_in_unit_disk;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub look_at_axis: Vec3,
    pub vertical_axis: Vec3,
    pub horizontal_axis: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: Point3,
               look_at: Point3,
               vup: Vec3,
               vertical_fov: f64,
               aspect_ratio: f64,
               aperture: f64,
               focus_distance: f64
        ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let look_at_axis = (look_from - look_at).unit();
        let horizontal_axis = vup.cross(look_at_axis).unit();
        let vertical_axis = look_at_axis.cross(horizontal_axis);

        let origin = look_from;
        let vertical = focus_distance * viewport_height * vertical_axis;
        let horizontal = focus_distance * viewport_width * horizontal_axis;
        let lower_left_corner = origin 
                                      - horizontal/2.0
                                      - vertical/2.0
                                      - focus_distance * look_at_axis;

        let lens_radius = aperture / 2.0;

        Camera { 
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            look_at_axis,
            vertical_axis,
            horizontal_axis,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.horizontal_axis * rd.x() + self.vertical_axis * rd.y();

        let direction =
            self.lower_left_corner
            + s * self.horizontal
            + t * self.vertical
            - self.origin
            - offset;

        Ray::new(
            self.origin + offset, 
            direction
        )
    }
}