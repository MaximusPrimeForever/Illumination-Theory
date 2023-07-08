mod ray;
mod vec3;
mod color;
mod sphere;
mod camera;
mod hittable;
mod rtweekend;
mod hittable_group;

use std::rc::{Rc};
use std::sync::Arc;

use sphere::Sphere;
use camera::Camera;
use vec3::{Point3, Color};

use hittable_group::HittableGroup;
use crate::color::{ray_color, write_color};

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};


fn main() {
    // Image
    let img_aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / img_aspect_ratio) as i32;
    
    let samples_per_pixel = 100;
    let trace_depth = 5;

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
    let multi_bar = Arc::new(MultiProgress::new());
    let height_bar: ProgressBar = multi_bar.add(ProgressBar::new(image_height as u64));
    let width_bar: ProgressBar = multi_bar.add(ProgressBar::new(image_width as u64));
    
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    ).unwrap().progress_chars("##-");
    height_bar.set_style(sty.clone());
    width_bar.set_style(sty);

    // Header
    println!("P3\n{image_width} {image_height}\n{}\n", color::MAX_COLOR);
    
    // Pixels
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let mut pixel_color = Color::origin();

            for _ in 0..samples_per_pixel {
                let u = (j as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (i as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
            
                pixel_color += ray_color(&ray, &world, trace_depth);
            }
            write_color(pixel_color, samples_per_pixel);

            width_bar.set_message(format!("col #{}", j));
            width_bar.inc(1);
        }
        width_bar.finish();
        width_bar.reset();

        height_bar.set_message(format!("line #{}", image_height - i));
        height_bar.inc(1);
    }
    height_bar.finish();
    eprintln!("\nDone.");
}
