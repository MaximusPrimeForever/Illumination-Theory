#![allow(dead_code)]
/// This module contains mostly scene descriptions, and some util functions.

use std::fs::File;
use std::sync::Arc;

use rand::random;

use crate::{
    world::World,
    material::{Lambertian, Metal, Dielectric, MaterialSync},
    vec3::{Color, Point3, Vec3}, sphere::Sphere, light::Light,
    camera::Camera,
    render::render_scene,
    buffer::write_img_ppm, hittable::HittableSync, texture::{SolidColorTexture, CheckerTexture}
};


/// Generate a random number in a given half open range
/// [min, max)
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + random::<f64>() * (max - min)
}

pub fn test_scene() {
    // Camera    
    let mut cam = Camera::default();
    cam.look_from = Point3::new(0.0, 0.0, 1.0);
    cam.look_at = Point3::new(0.0, 0.0, -1.0);

    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let mut lights = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(
        Sphere::new_stationary(
            Point3::new(0.0, -1000.5, -1.0),
            1000.0, 
            ground_material
        )
    ));
    
    // big shiny ball
    let shiny = Metal{
        albedo: Color::new(1.4, 1.2, 1.0) * 0.5, 
        fuzz: 0.0
    };
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(shiny)
    )));
    
    lights.push(Arc::new(Light::new(
        Point3::new(-3.0, 5.0, 1.5),
        Color::new(1.0, 1.0, 1.0),
        1.0
    )));
    
    let world = World::new(objects, lights);
    // Must be called!
    cam.initialize();

    // Render
    let image_canvas = render_scene(
        1,
        Arc::new(world),
        Arc::new(cam),
        10,
        10
    );
    
    // Output to file
    let mut output_image_file = File::create("output.ppm").unwrap();
    write_img_ppm(image_canvas, &mut output_image_file);
}

pub fn one_weekend_endgame(cam: &mut Camera, grid_size: i32) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let lights = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(
        Sphere::new_stationary(
            Point3::new(0.0, -1000.0, 0.0), 
            1000.0, 
            ground_material
        )
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
                let sphere_material: Arc<MaterialSync>;
                let sphere: Sphere;

                // pick material
                if choose_material < 0.6 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));

                } else if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal{albedo, fuzz});

                } else {
                    sphere_material = Arc::new(Dielectric{ir: 1.5});
                }
                sphere = Sphere::new_stationary(center, 0.2, sphere_material);
                objects.push(Arc::new(sphere));
            }
        }
    }

    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric{ir: 1.5})
    )));
    
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))
    )));

    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));

    // Camera
    cam.defocus_angle = 0.6;
    cam.focus_dist = 13.38;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 1.5, 0.0);

    World::new(objects, lights)
}

pub fn cool_effects(sphere_count: u32, distance: f64) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let lights = Vec::new();

    // ground
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)))
    )));

    // big shiny ball
    let shiny = Metal{
        albedo: Color::new(1.4, 1.2, 1.0) * 0.5, 
        fuzz: 0.0
    };
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(shiny)
    )));

    let theta_rad = (2.0 * std::f64::consts::PI) / (sphere_count as f64);
    for i in 0..sphere_count {
        let x = distance * (theta_rad * i as f64).cos();
        let y = 0.2;
        let z = distance * (theta_rad * i as f64).sin();

        objects.push(Arc::new(Sphere::new_stationary(
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
        objects.push(Arc::new(Sphere::new_stationary(
            Point3::new(x, y, z),
            0.2, 
            Arc::new(Metal{albedo: albedo, fuzz: 0.0})
        )));
    }

    World::new(objects, lights)
}

pub fn row_of_glass(sphere_count: u32, distance: f64) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let lights = Vec::new();

    // ground
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)))
    )));

    for i in 0..sphere_count { 
        // let albedo = Color::random() * Color::random() * 2.0;
        objects.push(Arc::new(Sphere::new_stationary(
            Point3::new((0.1 + distance) * i as f64, 0.2, 0.0),
            0.2, 
            Arc::new(Dielectric{ir: 1.5})
        )));
    }

    let final_row_z = sphere_count as f64 * (0.2 + distance);
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(final_row_z, 0.2, 0.25),
        0.1,
        Arc::new(Lambertian::new(Color::new(2.0, 0.2, 0.2)))
    )));
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(final_row_z, 0.2, 0.0),
        0.1,
        Arc::new(Lambertian::new(Color::new(0.2, 2.0, 0.2)))
    )));
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(final_row_z, 0.2, -0.25),
        0.1,
        Arc::new(Lambertian::new(Color::new(0.2, 0.2, 2.0)))
    )));

    World::new(objects, lights)
}

pub fn grid_of_glass(size: u32, distance: f64, radius: f64) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let lights = Vec::new();

    // ground
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)))
    )));

    let total_width = (size - 1) as f64 * (radius + distance);
    let starting_point = Point3::new(
        -total_width / 2.0,
        radius + 0.5, 
        -total_width / 2.0
    );

    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, total_width / 2.0 + 1.0, 0.0),
        0.1,
        Arc::new(Lambertian::new(Color::new(4.0, 0.2, 0.2)))
    )));

    for y in 0..size { 
        for z in 0..size {
            for x in 0..size {
                // let albedo = Color::random() * Color::random() * 2.0;
                objects.push(Arc::new(Sphere::new_stationary(
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

    World::new(objects, lights)
}

pub fn lit_world(cam: &mut Camera) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let mut lights = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(
        Sphere::new_stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));

    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric{ir: 1.5})
    )));
    
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 2.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))
    )));

    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, -2.0),
        1.0,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));

    lights.push(Arc::new(Light::new(
        Point3::new(-3.0, 5.0, 1.5),
        Color::new(1.0, 1.0, 1.0),
        1.0
    )));

    // Camera
    cam.defocus_angle = 0.6;
    cam.focus_dist = 13.38;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 1.5, 0.0);
    
    World::new(objects, lights)
}

pub fn lit_world_textures(cam: &mut Camera) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let mut lights = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(
        Sphere::new_stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));

    let solid_texture = SolidColorTexture::new(
        Color::new(0.4, 0.2, 0.1)
    );
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 2.0),
        1.0,
        Arc::new(Lambertian::new_texture(Arc::new(solid_texture))))
    ));
    
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));
    
    let checkered = CheckerTexture::new_color(
        0.3,
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.2, 0.3, 0.1),
    );
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, -2.0),
        1.0,
        Arc::new(Lambertian::new_texture(Arc::new(checkered)))
    )));

    lights.push(Arc::new(Light::new(
        Point3::new(-3.0, 5.0, 1.5),
        Color::new(1.0, 1.0, 1.0),
        1.0
    )));

    // Camera
    cam.defocus_angle = 0.6;
    cam.focus_dist = 13.38;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 1.5, 0.0);
    
    World::new(objects, lights)
}

pub fn two_checkered_spheres(cam: &mut Camera) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    
    let checkered = CheckerTexture::new_color(
        0.3,
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.2, 0.3, 0.1),
    );
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(Arc::new(checkered.clone())))
    )));
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(Arc::new(checkered)))
    )));

    // Camera
    cam.defocus_angle = 0.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    
    World::new_objects_only(objects)
}

pub fn one_weekend_motion_blur(grid_size: i32) -> World {
    let mut objects: Vec<Arc<HittableSync>> = Vec::new();
    let lights = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(
        Sphere::new_stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
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
                let sphere_material: Arc<MaterialSync>;
                let sphere: Sphere;

                // pick material
                let mut is_moving = false;
                if choose_material < 0.6 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    is_moving = true;

                } else if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal{albedo, fuzz});

                } else {
                    sphere_material = Arc::new(Dielectric{ir: 1.5});
                }

                if is_moving {
                    let direction = Vec3::new(
                        0.0, 
                        random_f64_in_range(0.0, 0.5), 
                        0.0
                    );
                    sphere = Sphere::new_moving(center, 0.2, sphere_material, direction);
                } else {
                    sphere = Sphere::new_stationary(center, 0.2, sphere_material);
                }
                objects.push(Arc::new(sphere));
            }
        }
    }

    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric{ir: 1.5})
    )));
    
    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))
    )));

    objects.push(Arc::new(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal{albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0})
    )));

    World::new(objects, lights)
}