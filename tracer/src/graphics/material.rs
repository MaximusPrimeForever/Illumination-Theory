use std::sync::Arc;

use rand::random;

use crate::ray::Ray;
use crate::geometry::hittable::HitRecord;
use crate::math::vec3::{Vec3, Color, Point3};
use crate::rendering::color::{COLOR_WHITE, COLOR_BLACK};
use crate::math::optics::{reflect, refract};
use crate::graphics::texture::{TextureSync, SolidColorTexture};

use crate::utils::{
    get_random_point_in_unit_sphere,
    get_random_point_on_unit_sphere
};


pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)>;

    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        COLOR_BLACK
    }
}

// ? wtf is this, read about it
pub type MaterialSync = dyn Material + Send + Sync;


// Diffuse
// =======
pub struct Lambertian {
    albedo: Arc<TextureSync>
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Lambertian { albedo: Arc::new(SolidColorTexture::new(color)) }
    }

    pub fn new_texture(t: Arc<TextureSync>) -> Self {
        Lambertian { albedo: t }
    }
}

impl Material for Lambertian {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = diffuse_lambertian_reflection(hitrec.normal);

        // Avoid situation where scatter direction vector is zero
        if scatter_direction.near_zero() { scatter_direction = hitrec.normal; }

        Some((
            self.albedo.value(hitrec.u, hitrec.v, &hitrec.point), 
            Ray::new(hitrec.point, scatter_direction, incident_ray.time)
        ))
    }
}

// These functions shoot the ray in some random direction
// by adding the normal vector the target point is displaced
// in a direction determined by the surface's orientation.
#[allow(dead_code)]
fn diffuse_rejection_method(normal: Vec3) -> Vec3 {
    normal + get_random_point_in_unit_sphere()
}

#[allow(dead_code)]
fn diffuse_lambertian_reflection(normal: Vec3) -> Vec3 {
    normal + get_random_point_on_unit_sphere()
}

#[allow(dead_code)]
fn diffuse_uniform_without_normal(normal: Vec3) -> Vec3 {
    let in_unit_sphere = get_random_point_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        in_unit_sphere
    }
}

// Mirror
// =======

#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }

    pub fn new_mirror(albedo: Color) -> Self {
        Metal { albedo, fuzz: 0.0 }
    }

    pub fn new_fuzzy(albedo: Color) -> Self {
        Metal { albedo, fuzz: 1.0 }
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_direction = reflect(incident_ray.direction.unit(), hitrec.normal);
        let scattered_ray = Ray::new(
            hitrec.point, 
            reflected_direction + self.fuzz * get_random_point_in_unit_sphere(), 
            incident_ray.time
        );

        if scattered_ray.direction.dot(hitrec.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }
}

// Dialectics
// ==========
#[derive(Default)]
pub struct Dielectric {
    pub ir: f64 // index of refraction
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 -  ref_idx) / (1.0 + ref_idx)).powi(2);
    
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, incident_ray: &Ray, hitrec: &HitRecord) -> Option<(Color, Ray)> {
        // If the ray comes from the outside, assume the refraction index outside the 
        // object is 1.0
        let refraction_ratio = if hitrec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = incident_ray.direction.unit();
        let cos_theta = -unit_direction.dot(hitrec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction;
        // if ray cannot refract, it gets reflected
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let some_bullshit = reflectance(cos_theta, refraction_ratio) > random::<f64>();

        if cannot_refract || some_bullshit {
            direction = reflect(unit_direction, hitrec.normal);
        } else {
            // TODO: Return 2 rays maybe? one reflects, one refracts
            // play with random weights for each ray's attenutation
            direction = refract(unit_direction, hitrec.normal, refraction_ratio);
        }

        Some((
            COLOR_WHITE,
            Ray::new(hitrec.point, direction, incident_ray.time)
        ))
    }
}