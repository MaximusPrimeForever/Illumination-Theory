use crate::ray::Ray;
use crate::vec3::{get_random_point_on_unit_sphere, reflect, unit_vector, dot};
use crate::{hittable::HitRecord, vec3::Color};


pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)>;
}


// Diffuse
// =======

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hitrec.normal + get_random_point_on_unit_sphere();

        // Avoid situation where scatter direction vector is zero
        if scatter_direction.near_zero() { scatter_direction = hitrec.normal; }

        Some((self.albedo, Ray::new(hitrec.point, scatter_direction)))
    }
}

// Mirror
// =======

pub struct Metal {
    pub albedo: Color
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&unit_vector(incident_ray.direction), &hitrec.normal);
        let scattered = Ray::new(hitrec.point, reflected);

        if dot(&scattered.direction, &hitrec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}