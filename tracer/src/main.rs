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

use std::env;
use std::rc::Rc;
use std::fs::File;
use std::sync::Arc;

use sphere::Sphere;
use camera::Camera;
use vec3::{Point3, Color};
use buffer::{SliceBuffer, Canvas, write_img_ppm, render_slice};

use world::World;
use crate::color::{ray_color, write_color};
use crate::material::{Lambertian, Metal, Dialectic};
use crate::vec3::Vec3;

use indicatif::MultiProgress;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Invalid arguments");
    }

    let image_width = args[1].parse::<usize>().unwrap();
    let samples_per_pixel: u32 = args[2].parse::<u32>().unwrap();
    let trace_depth: i32 = args[3].parse::<i32>().unwrap();

    // Image
    let img_aspect_ratio: f64 = 16.0 / 9.0;
    let image_height: usize = (image_width as f64 / img_aspect_ratio) as usize;

    // World
    let mut world = World::default();
    
    // left
    world.add(Rc::new(Sphere::new(
        &Point3::new(-1.0, -0.1, -1.0),
        0.4,
        // Rc::new(Metal{albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.3})
        Rc::new(Metal{albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0})
    )));
    
    // center
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian{albedo: Color::new(0.1, 0.2, 0.5)})
        // Rc::new(Dialectic{ir: 1.5})
    )));
    
    // right
    world.add(Rc::new(Sphere::new(
        &Point3::new(1.0, -0.1, -1.0),
        -0.4,
        // Rc::new(Metal{albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0})
        Rc::new(Dialectic{ir: 1.5})
    )));

    world.add(Rc::new(Sphere::new(
        &Point3::new(-0.5, 0.6, -1.0),
        0.1,
        Rc::new(Lambertian{albedo: Color::new(2.0, 0.3, 0.3)})
    )));

    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian{albedo: Color::new(0.6, 0.8, 0.0)})
    )));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let vfov = 88.0;
    let cam = Camera::new(
        Point3::new(2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio
    );

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
