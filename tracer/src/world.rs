use std::rc::Rc;
use std::vec::Vec;

use crate::ray::Ray;
use crate::hittable::{HittableT, HitRecord};


/// This class optimizes intersections with a group of objects
/// It's more than simply a list of hittable objects
/// If one object is infront of another then the result of calling hit() with a
/// ray that potentially intersects with both, will return a HitRecord of the closest
/// one.
#[derive(Default)]
pub struct World {
    pub objects: Vec<Rc<dyn HittableT>>
}

impl World {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn HittableT>) {
        self.objects.push(object)
    }
}

impl HittableT for World {
    /// Iterates list of objects and tries to find the closest object a given ray intersects with.
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut final_hrec = None;
        let mut closest_so_far: f64 = t_max;

        for obj in &self.objects {
            match obj.hit(ray, t_min, closest_so_far) {
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    final_hrec = Some(hit_record);
                }
                None => {}
            }
        }

        final_hrec
    }
}