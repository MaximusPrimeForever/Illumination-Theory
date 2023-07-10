use crate::ray::Ray;
use crate::vec3::get_random_point_on_unit_sphere;
use crate::{hittable::HitRecord, vec3::Color};


pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hitrec.normal + get_random_point_on_unit_sphere();

        // Avoid situation where scatter direction vector is zero
        if scatter_direction.near_zero() { scatter_direction = hitrec.normal; }

        Some((self.albedo, Ray::new(hitrec.point, scatter_direction)))
    }
}