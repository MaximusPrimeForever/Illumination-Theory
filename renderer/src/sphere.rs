use crate::hittable;
use crate::vec3;
use crate::ray::Ray as Ray;

use vec3::Point3 as Point3;
use vec3::Vec3 as Vec3;
use hittable::HittableT;

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Sphere {
        Sphere { center: *center, radius: radius } 
    }
}

impl HittableT for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b: f64 = vec3::dot(&ray.direction, &oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
    
        if discriminant < 0.0 { return false; }
        let dscr_sqrt = discriminant.sqrt();
        let mut root = (-half_b - dscr_sqrt) / a;

        if root < t_min || t_max > root {
            root = (-half_b + dscr_sqrt) / a;
            if root < t_min || t_max > root { return false; }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}