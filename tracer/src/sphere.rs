use std::rc::Rc;

use crate::ray::Ray;
use crate::material::Material;
use crate::vec3::{Vec3, Point3};
use crate::hittable::{HittableT, HitRecord};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere { center: *center, radius, material } 
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b: f64 = ray.direction.dot(oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
    
        if discriminant < 0.0 { return None; }
        let dscr_sqrt = discriminant.sqrt();
        let mut root = (-half_b - dscr_sqrt) / a;

        if t_min > root || root > t_max {
            root = (-half_b + dscr_sqrt) / a;
            if t_min > root || root > t_max{ return None; }
        }

        let point = ray.at(root);
        let material_rc = Rc::clone(&self.material);
        let rec = HitRecord::new(
            point,
            (point - self.center) / self.radius,
            material_rc,
            root,
            *ray
        );

        Some(rec)
    }
}