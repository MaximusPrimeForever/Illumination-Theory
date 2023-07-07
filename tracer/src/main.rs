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

use sphere::Sphere;
use camera::Camera;
use vec3::{Point3, Color};

use hittable_group::HittableGroup;

use crate::color::{ray_color, write_color};


fn main() {
    let mut stderr = stderr();

    // Image
    let img_aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / img_aspect_ratio) as i32;
    let samples_per_pixel = 50;

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
    let cam = Camera::new();

    // Render

    // Header
    println!("P3\n{image_width} {image_height}\n{}\n", color::MAX_COLOR);
    
    // Pixels
    for i in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", i);
        stderr.flush().unwrap();

        for j in 0..image_width {
            let mut pixel_color = Color::origin();

            for _s in 0..samples_per_pixel {
                let u = (j as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (i as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);

                pixel_color += ray_color(&ray, &world);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
