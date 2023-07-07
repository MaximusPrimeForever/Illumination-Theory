use crate::ray::Ray;
use crate::hittable::{HittableT, HitRecord};
use crate::vec3::{Vec3, Point3, dot};

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Sphere {
        Sphere { center: *center, radius } 
    }
}

impl HittableT for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b: f64 = dot(&ray.direction, &oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
    
        if discriminant < 0.0 { return false; }
        let dscr_sqrt = discriminant.sqrt();
        let mut root = (-half_b - dscr_sqrt) / a;

        if t_min > root || root > t_max {
            root = (-half_b + dscr_sqrt) / a;
            if t_min > root || root > t_max{ return false; }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}