use std::vec::Vec;
use std::sync::Arc;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::light::Light;
use crate::vec3::{Point3, Color};
use crate::color::COLOR_BLACK;
use crate::hittable::{HittableSync, HitRecord};


/// This class optimizes intersections with a group of objects
/// It's more than simply a list of hittable objects
/// If one object is infront of another then the result of calling hit() with a
/// ray that potentially intersects with both, will return a HitRecord of the closest
/// one.
#[derive(Default)]
pub struct World {
    pub objects: Vec<Arc<HittableSync>>,
    pub lights: Vec<Arc<Light>>
}

impl World {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add_object(&mut self, object: Arc<HittableSync>) {
        self.objects.push(object)
    }

    pub fn add_light(&mut self, light: Arc<Light>) {
        self.lights.push(light)
    }

    /// Iterates list of objects and tries to find the closest object a given ray intersects with.
    pub fn hit_object(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let mut final_hrec = None;
        let mut closest_interval = ray_interval.clone();

        for obj in &self.objects {
            match obj.hit(ray, closest_interval) {
                Some(hit_record) => {
                    closest_interval.max = hit_record.t;
                    final_hrec = Some(hit_record);
                }
                None => {}
            }
        }

        final_hrec
    }

    /// Iterate all the lights in the world, and compute light and shadow rays
    /// 
    /// The sum of all shadow and light rays result in the final color.
    /// Light rays' color is determined by the light's color and brightness.
    /// Whereas the shadow ray just results in a black color.
    pub fn hit_lights(&self, point: Point3, t_min: f64) -> Color {
        let mut color = Color::zero();

        for light in &self.lights {
            // before setting the direction to be unit long
            // mirror shadows would appear sometimes
            let direction = (light.origin - point).unit();
            let ray = Ray::new(point, direction, 0.0);
            
            let t_max = (light.origin - point).length();
            match &self.hit_object(ray, Interval::new(t_min, t_max)) {
                // shadow rays are black - cuz i said so
                Some(_) => {
                    color += COLOR_BLACK;
                }
                None => { color += light.color * light.brightness }
            }
        }

        color
    }
}
