use std::sync::Arc;
use std::f64::consts::PI;

use crate::graphics::{material::MaterialSync, aabb::AABB};
use crate::math::{interval::Interval, vec3::{Vec3, Point3}};
use crate::geometry::{Ray, hittable::{Hittable, HitRecord}};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<MaterialSync>,
    pub bounding_box: AABB
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<MaterialSync>) -> Sphere {
        let radius_vec = Vec3::new(radius, radius, radius);

        Sphere {
            center: center,
            radius, material,
            bounding_box: AABB::new_from_points(center - radius_vec, center + radius_vec)
        } 
    }

    #[allow(dead_code)]
    pub fn at(&self) -> Point3 {
        self.center
    }

    /// p: a given point on the sphere of radius one, centered at the origin.
    /// u: returned value [0,1] of angle around the Y axis from X=-1.
    /// v: returned value [0,1] of angle from Y=-1 to Y=+1.
    ///     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    ///     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    ///     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn get_sphere_uv(&self, point: Point3) -> (f64, f64) {
        let theta = (-point.y()).acos();
        let phi = (-point.z()).atan2(point.x()) + PI;

        ((phi / (2.0 * PI)), (theta / PI))
    }
}

impl Hittable for Sphere {
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
    /// (A + t*d - C)(A + t*d - C) = r^2
    /// 
    /// Simplify, and we get a quadratic equation:
    /// t^2*d*d + t*2d*(A-C) + (A-C)*(A-C) - r^2 = 0
    /// 
    /// Solving for t (the only unknown) there is a square root part that is either:
    /// * positive - we get 2 intersection points.
    /// * zero - we get a single intersection point.
    /// * negative - we don't get an intersection point.
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let center: Point3 = self.center;

        let oc: Vec3 = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b: f64 = ray.direction.dot(oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
    
        if discriminant < 0.0 { return None; }
        let dscr_sqrt = discriminant.sqrt();
        let mut root = (-half_b - dscr_sqrt) / a;

        if !ray_interval.fully_contains(root) {
            root = (-half_b + dscr_sqrt) / a;
            if !ray_interval.fully_contains(root) { return None; }
        }

        let point = ray.at(root);
        let outward_normal = (point - center) / self.radius;
        let (u, v) = self.get_sphere_uv(outward_normal);

        let rec = HitRecord::new(
            point,
            outward_normal,
            self.material.clone(),
            root,
            u,
            v,
            ray
        );

        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}