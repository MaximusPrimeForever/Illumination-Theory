use rand::random;
use std::sync::Arc;

use crate::geometry::hittable::HittableSync;
use crate::graphics::material::{Isotropic, MaterialSync};
use crate::graphics::texture::TextureSync;
use crate::math::interval::{Interval, UNIVERSE};
use crate::math::vec3::{Color, Vec3};

use super::hittable::{HitRecord, Hittable};


#[derive(Clone)]
pub struct ConstantMedium {
    boundry: Arc<HittableSync>,
    neg_inv_density: f64,
    phase_function: Arc<MaterialSync>,
}

impl ConstantMedium {
    pub fn new_texture(boundry: Arc<HittableSync>, density: f64, texture: Arc<TextureSync>) -> Self {
        ConstantMedium { 
            boundry: boundry, 
            neg_inv_density: -1.0 / density, 
            phase_function: Arc::new(Isotropic::new_texture(texture))
        }
    }

    pub fn new_color(boundry: Arc<HittableSync>, density: f64, color: Color) -> Self {
        ConstantMedium { 
            boundry: boundry, 
            neg_inv_density: -1.0 / density, 
            phase_function: Arc::new(Isotropic::new_color(color))
        }
    }
}

impl Hittable for ConstantMedium {
    /// Find and compute hit point inside constant medium hittable.
    /// 2 hit points are computed - entry, and exit
    /// The distance between those 2 is calculated, and then based on the matter density
    /// A potential scatter point inside the medium is computed.
    fn hit(&self, ray: super::Ray, ray_interval: crate::math::interval::Interval) -> Option<super::hittable::HitRecord> {
        // Compute entry point
        let hitrec1_op: Option<HitRecord> = self.boundry.hit(ray, UNIVERSE);
        if hitrec1_op.is_none() { return None }

        // Compute exit point
        let mut hitrec1 = hitrec1_op.unwrap();
        let hitrec2_op: Option<HitRecord> = self.boundry.hit(ray, Interval::new(hitrec1.t + 0.0001, f64::INFINITY));
        if hitrec2_op.is_none() { return None }

        let mut hitrec2 = hitrec2_op.unwrap();
        if hitrec1.t < ray_interval.min { hitrec1.t = ray_interval.min}
        if hitrec2.t > ray_interval.max { hitrec2.t = ray_interval.max}

        if hitrec1.t >= hitrec2.t { return None }
        if hitrec1.t < 0.0 { hitrec1.t = 0.0 }

        // Find distance between entry and exit, and distance of travel with respect to medium density
        let ray_length = ray.direction.length();
        let distance_inside_boundry = (hitrec2.t - hitrec1.t) * ray_length;

        // the ln() returns a random distance between (-INF, 0)
        // Multiplied by the inverse matter density gives a distance where scattering occurs
        let hit_distance = self.neg_inv_density * f64::ln(random::<f64>());
        if hit_distance > distance_inside_boundry { return None }

        // Build a hit record for the hit point inside the medium
        let time = hitrec1.t + hit_distance / ray_length;
        let mut hitrec: HitRecord = HitRecord::new(
            ray.at(time), 
            Vec3::zero(), 
            self.phase_function.clone(), 
            time,
            0.0, // Didn't set those in the book for some reason?
            0.0, // Didn't set those in the book for some reason?
            ray
        );

        hitrec.front_face = true;

        Some(hitrec)
    }

    fn bounding_box(&self) -> crate::graphics::aabb::AABB {
        self.boundry.bounding_box()
    }
}