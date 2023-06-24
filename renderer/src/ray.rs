use crate::vec3;

use vec3::Vec3 as Vec3;
use vec3::Point3 as Point3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn zero() -> Ray { 
        Ray { 
            origin: Vec3::origin(),
            direction: Vec3::origin()
        } 
    }
    pub fn new(origin: &Point3, direction: &Vec3) -> Ray { 
        Ray { 
            origin: origin.clone(),
            direction: direction.clone() 
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    /* Compute intersection of given ray and given sphere.
    
    The sphere equation is:
    (x - Cx)^2 + (y - Cy)^2 + (z - Cz)^2 = r^2
    Given the sphere vector and a 3D point we can rewrite as:
    (P - C)*(P - C) = r^2
    
    We want to check if a given ray has a 't' for which it intersects with the sphere.
    So,
    (P(t) - C)(P(t) - C) = r^2
    (A + t*b - C)(A + t*b - C) = r^2
    Simplify, and we get a quadratic equation:
    t^2*b*b + t*2b*(A-C) + (A-C)*(A-C) - r^2 = 0

    Solving for t (the only unknown) there is a square root part that is either:
    * positive - we get 2 intersection points.
    * zero - we get a single intersection point.
    * negative - we don't get an intersection point.
     */
    let oc: Vec3 = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b: f64 = vec3::dot(ray.direction, oc);
    let c: f64 = oc.length_squared() - radius * radius;
    let discriminant: f64 = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}