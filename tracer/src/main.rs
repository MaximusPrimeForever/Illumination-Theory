mod ray;
mod vec3;
mod color;
mod world;
mod buffer;
mod sphere;
mod optics;
mod camera;
mod render;
mod material;
mod geometry;
mod hittable;
mod rtweekend;

use std::env;
use std::fs::File;
use std::sync::Arc;

use camera::Camera;
use render::render_scene;
use buffer::write_img_ppm;

use vec3::{Vec3, Point3, Color};
use crate::color::{ray_color, write_color};
use crate::rtweekend::random_scene;


fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Invalid arguments");
    }

    let image_width = args[1].parse::<usize>().unwrap();
    let samples_per_pixel: u32 = args[2].parse::<u32>().unwrap();
    let trace_depth: i32 = args[3].parse::<i32>().unwrap();
    let core_count: usize = args[4].parse::<usize>().unwrap();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // World
    let world = random_scene();
    
    // Camera
    let vfov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::origin();
    
    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio,
        0.1,
        10.0
    );

    let image_canvas = render_scene(
        core_count, 
        image_width,
        image_height,
        Arc::new(world),
        Arc::new(cam),
        samples_per_pixel,
        trace_depth
    );
    
    // File
    let mut output_image_file = File::create("output.ppm")?;
    write_img_ppm(image_canvas, &mut output_image_file);
    
    eprintln!("\nDone.");
    Ok(())
}
