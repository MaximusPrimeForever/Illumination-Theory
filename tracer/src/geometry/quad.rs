use std::sync::Arc;

use crate::{
    math::vec3::{Point3, Vec3},
    graphics::{aabb::AABB, material::MaterialSync}, hittable::{HittableT, HitRecord}
};

/// The quad is defined by a bottom left coordinate from origin
/// and 2 vectors u,v like so:
///     /\ (v)
///     /
///    /
/// Q * -------> u
/// Defined by the plane equation: Ax + By + Cz = D
/// The plane normal is (A, B, C)
/// Any point (v) on the plane satisfies n ⋅ v = D
pub struct Quad {
    bottom_left: Point3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    d: f64,
    w: Vec3,
    material: Arc<MaterialSync>,
    bounding_box: AABB
}

impl Quad {
    pub fn new(_bottom_left: Point3, _u: Vec3, _v: Vec3, _material: Arc<MaterialSync>) -> Self {
        let n = _u.cross(_v);
        let normal = n.unit();
        Quad { 
            bottom_left: _bottom_left,
            u: _u,
            v: _v,
            normal: normal,
            d: normal.dot(_bottom_left),
            w: n / n.dot(n),
            material: _material,
            bounding_box: AABB::new_from_points(_bottom_left, _bottom_left + _u + _v).pad()
        }
    }
}

impl HittableT for Quad {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
    
    /// the ray-plane intersection is defined as:
    /// t = (D - n ⋅ P) / (n ⋅ d)
    fn hit(&self, ray: crate::ray::Ray, ray_interval: crate::interval::Interval) -> Option<crate::hittable::HitRecord> {
        let denom = self.normal.dot(ray.direction);
        
        // n ⋅ d is close to zero which means ray is parallel to plane
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_interval.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.bottom_left;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if alpha < 0.0 || 1.0 < alpha || beta < 0.0 || 1.0 < beta {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.normal,
            self.material.clone(),
            t,
            alpha,
            beta,
            ray
        ))
    }
}