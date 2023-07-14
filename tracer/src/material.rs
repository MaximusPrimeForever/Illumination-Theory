use crate::color::COLOR_WHITE;
use crate::ray::Ray;
use crate::vec3::{
    get_random_point_on_unit_sphere,
    reflect,
    get_random_point_in_unit_sphere,
    Point3,
    Vec3, refract
};
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

// These functions shoot the ray in some random direction
// by adding the normal vector the target point is displaced
// in a direction determined by the surface's orientation.

fn diffuse_rejection_method(point: Point3, normal: Vec3) -> Vec3 {
    point + normal + get_random_point_in_unit_sphere()
}

fn diffuse_lambertian_reflection(point: Point3, normal: Vec3) -> Vec3 {
    point + normal + get_random_point_on_unit_sphere()
}

fn diffuse_uniform_without_normal(point: Point3, normal: Vec3) -> Vec3 {
    let in_unit_sphere = get_random_point_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        point + in_unit_sphere
    } else {
        point - in_unit_sphere
    }
}

// Mirror
// =======

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(incident_ray.direction.unit(), hitrec.normal);
        let scattered = Ray::new(hitrec.point, reflected + self.fuzz * get_random_point_in_unit_sphere());

        if scattered.direction.dot(hitrec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

// Dialectics
// ==========

pub struct Dialectic {
    pub ir: f64 // index of refraction
}

impl Material for Dialectic {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        // If the ray comes from the outside, assume the refraction index outside the 
        // object is 1.0
        let refraction_ratio = if hitrec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = incident_ray.direction.unit();
        let cos_theta = -unit_direction.dot(hitrec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction;
        // if ray cannot refract, it gets reflected
        if refraction_ratio * sin_theta > 1.0 {
            direction = reflect(unit_direction, hitrec.normal);
        } else {
            direction = refract(unit_direction, hitrec.normal, refraction_ratio);
        }

        Some((COLOR_WHITE.clone(), Ray::new(hitrec.point, direction)))
    }
}