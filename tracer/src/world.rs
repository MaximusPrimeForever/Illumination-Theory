use std::vec::Vec;
use std::sync::Arc;

use crate::ray::Ray;
use crate::math::interval::Interval;
use crate::graphics::{
    bvh::BVH,
    light::Light
};
use crate::math::vec3::{Point3, Color};
use crate::rendering::color::COLOR_BLACK;
use crate::geometry::hittable::{HittableSync, HitRecord, HittableT, HittableComposite};

// #[derive(Default)]
pub struct World {
    bvh: Option<BVH>,
    objects: Vec<Arc<HittableSync>>,
    pub lights: Vec<Arc<Light>>
}

impl World {
    /// Build a BVH around the objects, and store lights information.
    pub fn new() -> World {
        let objects: Vec<Arc<HittableSync>> = Vec::new();

        World { bvh: None, objects, lights: Vec::new()}
    }

    /// Build a BVH around the objects, and store lights information.
    pub fn new_from_objects(mut objects: Vec<Arc<HittableSync>>) -> World {
        let bvh = BVH::new_tree_random_axis(&mut objects);
        World { bvh: Some(bvh), objects, lights: Vec::new()}
    }

    /// Build BVH tree from objects
    pub fn initialize(&mut self) {
        self.bvh = Some(BVH::new_tree_random_axis(&mut self.objects));
    }

    pub fn add_hittable(&mut self, object: Arc<HittableSync>) {
        self.objects.push(object);
        self.bvh = None;
    }

    pub fn add_hittable_composite(&mut self, object: &dyn HittableComposite) {
        self.objects.append(&mut object.to_hittable());
        self.bvh = None;
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(Arc::new(light));
    }

    /// Uses Bound Volume Hierarchy to optimize intersection computations.
    pub fn shoot_ray(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        if self.bvh.is_none() {
            panic!("World is not initialized! You must call initialize() after adding object using add_* methods.");
        }

        let bvh = self.bvh.as_ref().unwrap();
        bvh.hit(ray, ray_interval)
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
            match &self.shoot_ray(ray, Interval::new(t_min, t_max)) {
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
