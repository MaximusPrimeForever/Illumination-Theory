mod ray;
mod vec3;
mod color;
mod buffer;
mod sphere;
mod camera;
mod material;
mod hittable;
mod rtweekend;
mod world;

use std::rc::{Rc};
use std::fs::File;
use std::sync::Arc;

use sphere::Sphere;
use camera::Camera;
use vec3::{Point3, Color};
use buffer::{SliceBuffer, Canvas, write_img_ppm, render_slice};

use world::World;
use crate::color::{ray_color, write_color};
use crate::material::Lambertian;

use indicatif::{MultiProgress};

fn main() -> std::io::Result<()>{
    // Image
    let img_aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / img_aspect_ratio) as usize;
    
    let samples_per_pixel = 200;
    let trace_depth: i32 = 10;

    // World
    let mut world = World::default();
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.5, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian{albedo: Color::new(1.0, 0.0, 0.0)})
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(-0.5, -0.2, -1.0),
        0.3,
        Rc::new(Lambertian{albedo: Color::new(0.0, 1.0, 0.0)})
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -0.4, -1.0),
        0.1,
        Rc::new(Lambertian{albedo: Color::new(1.0, 0.0, 0.0)})
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(-0.5, 2.5, -1.0),
        2.0,
        Rc::new(Lambertian{albedo: Color::new(1.0, 0.0, 0.0)})
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian{albedo: Color::new(1.0, 1.0, 1.0)})
    )));

    // Camera
    let cam = Camera::new();

    // Render
    let multi_bar = Arc::new(MultiProgress::new());
    let mut image_canvas = Canvas::new(image_width, image_height);

    let mut main_slice = SliceBuffer{height: image_height, width: image_width, ..Default::default()};
    render_slice(&mut main_slice, world, cam, samples_per_pixel, trace_depth, multi_bar);
    image_canvas.write_slice(main_slice);
    
    // File
    let mut output_image_file = File::create("output.ppm")?;
    write_img_ppm(image_canvas, &mut output_image_file);
    
    eprintln!("\nDone.");
    Ok(())
}
