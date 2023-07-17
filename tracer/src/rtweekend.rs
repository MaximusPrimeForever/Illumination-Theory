use std::sync::Arc;

use rand::random;

use crate::{
    world::World,
    material::{Lambertian, Metal, Dielectric, MaterialSend},
    vec3::{Color, Point3}, sphere::Sphere
};

/// Generate a random number in a given half open range
/// [min, max)
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + random::<f64>() * (max - min)
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min { return min; }
    if value > max { return max; }
    value
}

pub fn random_scene() -> World {
    let mut world = World::default();

    let ground_material = Arc::new(Lambertian{albedo: Color::new(0.5, 0.5, 0.5)});
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));

    let some_point = Point3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>()
            );

            if (center - some_point).length() > 0.9 {
                let sphere_material: Arc<MaterialSend>;
                let sphere: Sphere;

                // pick material
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian{albedo});

                } else if choose_material < 0.95 {
                    let albedo = Color::random() * Color::random();
                    let fuzz = random_f64_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal{albedo, fuzz});

                } else {
                    sphere_material = Arc::new(Dielectric{ir: 1.5});
                }
                sphere = Sphere::new(center, 0.2, sphere_material);
                world.add(Arc::new(sphere));
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric{ir: 1.5})
    )));
    
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian{albedo: Color::new(0.4, 0.2, 0.1)})
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));

    world
}