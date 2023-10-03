
use std::sync::Arc;

use crate::ray::Ray;
use crate::interval::Interval;
use crate::math::vec3::{Point3, Vec3};
use crate::graphics::{material::Material, aabb::AABB};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64, // row of texture coordinate
    pub v: f64, // column of texture coordinate
    pub front_face: bool
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, material: Arc<dyn Material>, t: f64, u: f64, v: f64, ray: Ray) -> HitRecord {
        let (front_face, outward_normal) = set_face_normal(ray, normal);
        HitRecord { point, normal: outward_normal, material, t, u, v, front_face }
    }
}

/// Sets the normal to always face away from the surface the ray hit
fn set_face_normal(ray: Ray, normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction.dot(normal) < 0.0;
    let normal = if front_face { normal } else { -normal };

    (front_face, normal)
}

pub trait HittableT {
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

// ? wtf is this, read about it
pub type HittableSync = dyn HittableT + Send + Sync;
