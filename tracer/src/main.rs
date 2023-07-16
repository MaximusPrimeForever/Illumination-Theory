mod ray;
mod vec3;
mod color;
mod world;
mod buffer;
mod sphere;
mod optics;
mod camera;
mod material;
mod geometry;
mod hittable;
mod rtweekend;

use std::env;
use std::thread;
use std::fs::File;
use std::sync::Arc;

use camera::Camera;
use vec3::{Point3, Color};
use buffer::{SliceBuffer, Canvas, write_img_ppm, render_slice};

use world::World;
use crate::color::{ray_color, write_color};
use crate::rtweekend::random_scene;
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
    let aspect_ratio = 3.0 / 2.0;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    let slice_height = image_height;
    let slice_width = image_width / 4;
    let mut image_canvas = Canvas::new(image_width, image_height);
    
    // World
    let world = Arc::new(random_scene());
    
    // Camera
    let vfov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::origin();
    
    let cam = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio,
        0.0,
        10.0
    ));
    
    // Render
    let multi_bar = Arc::new(MultiProgress::new());

    let mut slice_1 = SliceBuffer::new_slice(
        slice_width,
        slice_height,
        0,
        0
    );
    let mut slice_2 = SliceBuffer::new_slice(
        slice_width,
        slice_height,
        0,
        slice_width * 1
    );
    let mut slice_3 = SliceBuffer::new_slice(
        slice_width,
        slice_height,
        0,
        slice_width * 2
    );
    let mut slice_4 = SliceBuffer::new_slice(
        slice_width,
        slice_height,
        0,
        slice_width * 3
    );

    // thread::scope(|scope| {
    //     scope.spawn(|_| {
    //         let a = 0;
    //         a
    //     }).join().unwrap()
    // }).unwrap();
    render_slice(&mut slice_1, image_width,  image_height, world.clone(), cam.clone(), samples_per_pixel, trace_depth, multi_bar.clone());
    
    render_slice(&mut slice_2, image_width, image_height, world.clone(), cam.clone(), samples_per_pixel, trace_depth, multi_bar.clone());
    
    render_slice(&mut slice_3, image_width, image_height, world.clone(), cam.clone(), samples_per_pixel, trace_depth, multi_bar.clone());
    
    render_slice(&mut slice_4, image_width, image_height, world.clone(), cam.clone(), samples_per_pixel, trace_depth, multi_bar.clone());
    
    image_canvas.write_slice(slice_1);
    image_canvas.write_slice(slice_2);
    image_canvas.write_slice(slice_3);
    image_canvas.write_slice(slice_4);

    // File
    let mut output_image_file = File::create("output.ppm")?;
    write_img_ppm(image_canvas, &mut output_image_file);
    
    eprintln!("\nDone.");
    Ok(())
}
