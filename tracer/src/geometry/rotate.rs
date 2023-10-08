/// A wrapper for Hittable that can rotate it
/// without actually rotating it in the world space.
/// It augments the incident ray and resulting intersecion point instead.

use std::sync::Arc;

use crate::geometry::Ray;
use crate::graphics::aabb::AABB;
use crate::math::{vec3::Vec3, interval::Interval};

use super::hittable::{Hittable, HitRecord, HittableSync};

/// "Rotate" a hittable around Y axis
/// The hittable isn't actually rotated, but rather the incident rays are rotated
pub struct RotateY {
    object: Arc<HittableSync>,
    theta: f64,
    bounding_box: AABB
}

impl RotateY {
    pub fn new(object: Arc<HittableSync>, angle: f64) -> Self {
        let bbox = object.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        // Find min, max points of bounding box post rotation
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;
                    
                    let tester = Vec3::new(x, y, z).rotate_y(angle.to_radians());

                    for i in 0..3 {
                        min[i] = min[i].min(tester[i]);
                        max[i] = max[i].max(tester[i]);
                    }
                }
            }
        }

        RotateY { 
            object,
            theta: angle,
            bounding_box: AABB::new_from_points(min, max) 
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }

    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        // Change ray from world space to object space
        let rotated_ray = Ray::new(
            ray.origin.rotate_y(-self.theta.to_radians()),
            ray.direction.rotate_y(-self.theta.to_radians()),
            ray.time
        );

        // Determine where (if any) an intersection occurs in object space
        let hitrec_result = self.object.hit(rotated_ray, ray_interval);
        if hitrec_result.is_none() { return None; }

        // Change intersection point and normal from object space to world space
        let mut hitrec = hitrec_result.unwrap();
        hitrec.point = hitrec.point.rotate_y(self.theta.to_radians());
        hitrec.normal = hitrec.normal.rotate_y(self.theta.to_radians());

        Some(hitrec)
    }
}