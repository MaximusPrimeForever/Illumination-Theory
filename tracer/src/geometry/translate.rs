use std::sync::Arc;

use crate::{math::{vec3::Vec3, interval::Interval}, graphics::aabb::AABB, ray::Ray};

use super::hittable::{HittableT, HitRecord, HittableSync};


pub struct Translate {
    object: Arc<HittableSync>,
    offset: Vec3,
    bounding_box: AABB
}

impl Translate {
    pub fn new(object: Arc<HittableSync>, displacement: Vec3) -> Self {
        let new_bbox = object.bounding_box() + displacement;

        Translate { 
            object, 
            offset: displacement,
            bounding_box: new_bbox
        }
    }
}

impl HittableT for Translate {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }

    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let ray_offset = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        let hitrec_result = self.object.hit(ray_offset, ray_interval);
        if hitrec_result.is_none() { return None; }

        let mut hitrec = hitrec_result.unwrap();
        hitrec.point += self.offset;

        Some(hitrec)
    }
}