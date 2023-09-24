mod ray;
mod vec3;
mod color;
mod world;
mod light;
mod buffer;
mod sphere;
mod optics;
mod camera;
mod render;
mod interval;
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


fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        rtweekend::test_scene();
        return Ok(());
    }
    if args.len() != 7 {
        panic!("Invalid arguments");
    }

    let image_width = args[1].parse::<usize>().unwrap();
    let aspect_ratio = args[2].parse::<f64>().unwrap();
    let vfov = args[3].parse::<f64>().unwrap();
    let samples_per_pixel: usize = args[4].parse::<usize>().unwrap();
    let trace_depth: usize = args[5].parse::<usize>().unwrap();
    let core_count: usize = args[6].parse::<usize>().unwrap();

    // World
    // let world = rtweekend::cool_effects(8, 1.3);
    let world = rtweekend::random_scene(8);
    
    // Camera    
    let mut cam = Camera::default();
    cam.image_width = image_width;
    cam.image_height = (image_width as f64 / aspect_ratio) as usize;
    cam.vfov = vfov;
    cam.look_from = Point3::new(0.0, 0.0, 1.0);
    cam.look_at = Point3::new(0.0, 0.0, -1.0);
    
    // Must be called!
    cam.initialize();

    // Render
    let image_canvas = render_scene(
        core_count,
        Arc::new(world),
        Arc::new(cam),
        samples_per_pixel,
        trace_depth
    );
    
    // Output to file
    let mut output_image_file = File::create("output.ppm")?;
    write_img_ppm(image_canvas, &mut output_image_file);
    
    eprintln!("\nDone.");
    Ok(())
}
