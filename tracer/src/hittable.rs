
use std::rc::Rc;

use crate::ray::Ray;
use crate::material::Material;
use crate::vec3::{Point3, Vec3};

// #[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, material: Rc<dyn Material>, t: f64, ray: Ray) -> HitRecord {
        let (front_face, outward_normal) = set_face_normal(ray, normal);
        HitRecord { point, normal: outward_normal, material, t, front_face }
    }
}

fn set_face_normal(ray: Ray, normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction.dot(normal) < 0.0;
    let normal = if front_face { normal } else { -normal };

    (front_face, normal)
}

pub trait HittableT {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
