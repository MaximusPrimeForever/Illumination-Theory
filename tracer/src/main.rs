mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_group;
mod rtweekend;
mod sphere;
mod camera;

use std::io::{Write, stderr};
use std::rc::Rc;

use vec3::{Vec3, Point3, Color};
use ray::Ray as Ray;

use hittable_group::HittableGroup;
use sphere::Sphere;

fn main() {
    let mut stderr = stderr();

    // Image
    let img_aspect_ratio: f64 = 16.0 / 9.0;
    let img_width: i32 = 800;
    let img_height: i32 = (img_width as f64 / img_aspect_ratio) as i32;


    // World
    let mut world: HittableGroup = HittableGroup::default();
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0
    )));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = img_aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::origin();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render

    // Header
    println!("P3\n{img_width} {img_height}\n{}\n", color::MAX_COLOR);
    
    // Pixels
    for i in (0..img_height).rev() {
        eprint!("\rScanlines remaining: {}", i);
        stderr.flush().unwrap();

        for j in 0..img_width {
            let u = (j as f64) / ((img_width - 1) as f64);
            let v = (i as f64) / ((img_height - 1) as f64);
            let dir = lower_left_corner + (u * horizontal) + (v * vertical) - origin;

            let ray = Ray::new(
                &origin,
                &dir
            );

            let pixel_color: Color = color::ray_color(&ray, &world);
            color::write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}