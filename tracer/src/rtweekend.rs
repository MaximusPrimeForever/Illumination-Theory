#![allow(dead_code)]
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

pub fn random_scene(grid_size: i32) -> World {
    let mut world = World::default();

    let ground_material = Arc::new(Lambertian{albedo: Color::new(0.5, 0.5, 0.5)});
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));

    let some_point = Point3::new(4.0, 0.2, 0.0);

    for a in -grid_size..grid_size {
        for b in -grid_size..grid_size {
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
                if choose_material < 0.6 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian{albedo});

                } else if choose_material < 0.8 {
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

pub fn cool_effects(sphere_count: u32, distance: f64) -> World {
    let mut world = World::default();

    // ground
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian{albedo: Color::new(0.5, 0.5, 0.5)})
    )));

    // big shiny ball
    let shiny = Metal{
        albedo: Color::new(1.4, 1.2, 1.0) * 0.5, 
        fuzz: 0.0
    };
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(shiny)
    )));

    let theta_rad = (2.0 * std::f64::consts::PI) / (sphere_count as f64);
    for i in 0..sphere_count { 
        let x = distance * (theta_rad * i as f64).cos();
        let y = 0.2;
        let z = distance * (theta_rad * i as f64).sin();

        world.add(Arc::new(Sphere::new(
            Point3::new(x, y, z),
            0.2, 
            Arc::new(Dielectric{ir: 1.5})
        )));
    }

    let theta_rad = (2.0 * std::f64::consts::PI) / (sphere_count as f64);
    for i in 0..sphere_count { 
        let x = (distance) * (theta_rad * 1.5 * i as f64).cos();
        let y = 0.2;
        let z = (distance) * (theta_rad * 1.5 * i as f64).sin();

        let albedo = Color::random() * Color::random() * 2.0;
        world.add(Arc::new(Sphere::new(
            Point3::new(x, y, z),
            0.2, 
            Arc::new(Metal{albedo: albedo, fuzz: 0.0})
        )));
    }

    world
}

pub fn row_of_glass(sphere_count: u32, distance: f64) -> World {
    let mut world = World::default();

    // ground
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian{albedo: Color::new(0.5, 0.5, 0.5)})
    )));

    for i in 0..sphere_count { 
        // let albedo = Color::random() * Color::random() * 2.0;
        world.add(Arc::new(Sphere::new(
            Point3::new((0.1 + distance) * i as f64, 0.2, 0.0),
            0.2, 
            Arc::new(Dielectric{ir: 1.5})
        )));
    }

    let final_row_z = sphere_count as f64 * (0.2 + distance);
    world.add(Arc::new(Sphere::new(
        Point3::new(final_row_z, 0.2, 0.25),
        0.1,
        Arc::new(Lambertian{albedo: Color::new(2.0, 0.2, 0.2)})
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(final_row_z, 0.2, 0.0),
        0.1,
        Arc::new(Lambertian{albedo: Color::new(0.2, 2.0, 0.2)})
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(final_row_z, 0.2, -0.25),
        0.1,
        Arc::new(Lambertian{albedo: Color::new(0.2, 0.2, 2.0)})
    )));

    world
}

pub fn grid_of_glass(size: u32, distance: f64, radius: f64) -> World {
    let mut world = World::default();

    // ground
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian{albedo: Color::new(0.5, 0.5, 0.5)})
    )));

    let total_width = (size - 1) as f64 * (radius + distance);
    let starting_point = Point3::new(
        -total_width / 2.0,
        radius + 0.5, 
        -total_width / 2.0
    );

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, total_width / 2.0 + 1.0, 0.0),
        0.1,
        Arc::new(Lambertian{albedo: Color::new(4.0, 0.2, 0.2)})
    )));

    for y in 0..size { 
        for z in 0..size {
            for x in 0..size {
                // let albedo = Color::random() * Color::random() * 2.0;
                world.add(Arc::new(Sphere::new(
                    Point3::new(
                        starting_point.x() + x as f64 * (radius + distance),
                        starting_point.y() + y as f64 * (radius + distance),
                        starting_point.z() + z as f64 * (radius + distance)
                    ),
                    radius, 
                    Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
                )));
            }
        }
    }

    world
}