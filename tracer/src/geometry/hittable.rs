
use std::vec::Vec;
use std::sync::Arc;

use crate::ray::Ray;
use crate::math::interval::Interval;
use crate::math::vec3::{Point3, Vec3};
use crate::graphics::{material::Material, aabb::AABB};

use crate::graphics::light::Light;
use crate::math::vec3::Color;
use crate::rendering::color::COLOR_BLACK;

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

/// A Hittable container for objects.
pub struct HittableComposite {
    bbox: AABB,
    pub objects: Vec<Arc<HittableSync>>,
    pub lights: Vec<Arc<Light>>
}

impl HittableT for HittableComposite {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
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
}

impl HittableComposite {
    pub fn new() -> Self {
        let objects: Vec<Arc<HittableSync>> = Vec::new();
        let bbox = AABB::default();

        HittableComposite { bbox, objects, lights: Vec::new()}
    }

    pub fn new_from_objects(objects: Vec<Arc<HittableSync>>) -> Self {
        let bbox = AABB::new_from_hittables(&objects);

        HittableComposite { bbox, objects, lights: Vec::new()}
    }

    pub fn add_hittable(&mut self, object: Arc<HittableSync>) {
        self.bbox += object.bounding_box();
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(Arc::new(light));
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
            match &self.hit(ray, Interval::new(t_min, t_max)) {
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
