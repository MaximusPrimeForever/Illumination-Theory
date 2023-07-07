
use crate::ray;
use crate::vec3;

use vec3::Point3 as Point3;
use vec3::Vec3 as Vec3;
use ray::Ray as Ray;

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = vec3::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { (-1.0) * (*outward_normal) };
    }
}

pub trait HittableT {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
