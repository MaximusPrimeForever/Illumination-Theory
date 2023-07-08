use std::vec::Vec;
use std::sync::Arc;

use crate::ray::Ray;
use crate::hittable::{HittableT, HitRecord};


/// This class optimizes intersections with a group of objects
/// It's more than simply a list of hittable objects
/// If one object is infront of another then the result of calling hit() with a
/// ray that potentially intersects with both, will return a HitRecord of the closest
/// one.
#[derive(Default)]
pub struct HittableGroup {
    pub objects: Vec<Arc<dyn HittableT>>
}

impl HittableGroup {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<dyn HittableT>) {
        self.objects.push(object)
    }
}

impl HittableT for HittableGroup {
    /// Iterates list of objects and tries to find the closest object a given ray intersects with.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut final_hrec = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for obj in &self.objects {
            match obj.hit(ray, t_min, closest_so_far) {
                Some(hit_record) => {
                    hit_anything = true;
                    closest_so_far = hit_record.t;
                    final_hrec = hit_record;
                }
                None => {}
            }
        }

        if !hit_anything { return None; }
        Some(final_hrec)
    }
}