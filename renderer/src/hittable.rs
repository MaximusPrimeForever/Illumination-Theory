
use crate::ray;
use crate::vec3;

use vec3::Point3 as Point3;
use vec3::Vec3 as Vec3;
use ray::Ray as Ray;

pub struct hit_record {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64
}

pub trait hittable_t {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
}
