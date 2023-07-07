use std::rc::Rc;
use std::vec::Vec;
use crate::hittable::{HittableT, HitRecord};


/// This class optimizes intersections with a group of objects
/// It's more than simply a list of hittable objects
/// If one object is infront of another then the result of calling hit() with a
/// ray that potentially intersects with both, will return a HitRecord of the closest
/// one.
#[derive(Default)]
pub struct HittableGroup {
    pub objects: Vec<Rc<dyn HittableT>>
}

impl HittableGroup {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn HittableT>) {
        self.objects.push(object)
    }
}

impl HittableT for HittableGroup {
    /// Iterates list of objects and tries to find the closest object a given ray intersects with.
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let mut temp_hrec: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_hrec) {
                hit_anything = true;
                closest_so_far = temp_hrec.t;
                *rec = temp_hrec;
            }
        }

        hit_anything
    }
}