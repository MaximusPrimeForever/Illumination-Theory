use std::sync::Arc;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::MaterialSend;
use crate::vec3::{Vec3, Point3};
use crate::hittable::{HittableT, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<MaterialSend>,
    is_moving: bool,
    movement_direction: Vec3,
    bounding_box: AABB
}

impl Sphere {
    pub fn new_stationary(center: Point3, radius: f64, material: Arc<MaterialSend>) -> Sphere {
        let radius_vec = Vec3::new(radius, radius, radius);

        Sphere {
            center: center,
            radius, material,
            is_moving: false,
            movement_direction: Vec3::zero(),
            bounding_box: AABB::new_from_points(center - radius_vec, center + radius_vec)
        } 
    }

    /// Moving sphere has an empty bounding box
    /// because I chose to use a direction vector to compute motion blur
    /// where as Peter decided to use 2 center points which the sphere moves between
    /// I think Peter's approach is weird and hardcoded, thus decided not to do it like him
    /// Besides, adding motion blur seems unnecessary
    pub fn new_moving(center: Point3, radius: f64, material: Arc<MaterialSend>, direction: Vec3) -> Sphere {
        Sphere {
            center: center,
            radius, material,
            is_moving: true,
            movement_direction: direction,
            bounding_box: AABB::default()
        }
    }

    pub fn at(&self, time: f64) -> Point3 {
        self.center + time * self.movement_direction
    }
}

impl HittableT for Sphere {
    /// Compute intersection of given ray and given sphere.
    ///
    /// The sphere equation is:
    /// 
    /// (x - Cx)^2 + (y - Cy)^2 + (z - Cz)^2 = r^2
    /// 
    /// Given the sphere vector and a 3D point we can rewrite as:
    /// 
    /// (P - C)*(P - C) = r^2
    /// 
    /// We want to check if a given ray has a 't' for which it intersects with the sphere.
    /// 
    /// So,
    /// (P(t) - C)(P(t) - C) = r^2
    /// (A + t*b - C)(A + t*b - C) = r^2
    /// 
    /// Simplify, and we get a quadratic equation:
    /// t^2*b*b + t*2b*(A-C) + (A-C)*(A-C) - r^2 = 0
    /// 
    /// Solving for t (the only unknown) there is a square root part that is either:
    /// * positive - we get 2 intersection points.
    /// * zero - we get a single intersection point.
    /// * negative - we don't get an intersection point.
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let center: Point3;
        if self.is_moving {
            center = self.at(ray.time);
        } else {
            center = self.center;
        }

        let oc: Vec3 = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b: f64 = ray.direction.dot(oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
    
        if discriminant < 0.0 { return None; }
        let dscr_sqrt = discriminant.sqrt();
        let mut root = (-half_b - dscr_sqrt) / a;

        if !ray_interval.surrounds(root) {
            root = (-half_b + dscr_sqrt) / a;
            if !ray_interval.surrounds(root) { return None; }
        }

        let point = ray.at(root);
        let material_rc = Arc::clone(&self.material);
        let rec = HitRecord::new(
            point,
            (point - center) / self.radius,
            material_rc,
            root,
            ray
        );

        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}