mod ray;
mod vec3;
mod color;
mod buffer;
mod sphere;
mod camera;
mod hittable;
mod rtweekend;
mod hittable_group;

use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::fs::File;
use std::sync::{Arc, mpsc};

use sphere::Sphere;
use camera::Camera;
use vec3::{Point3, Color};
use buffer::{SliceBuffer, Canvas, write_img_ppm, render_slice};

use hittable_group::HittableGroup;
use crate::color::{ray_color, write_color};

use indicatif::{MultiProgress};

fn render_in_thread(image_height: usize, image_width: usize, world_arc: Arc<HittableGroup>, cam: Camera, samples_per_pixel: u32, trace_depth: i32, multi_bar: Arc<MultiProgress>) -> JoinHandle<()> {
    let (sender, receiver) = mpsc::channel::<SliceBuffer>();
    
    let handle =thread::spawn(|| {
        let mut slice_buffer = SliceBuffer{
            height: image_height, 
            width: image_width, 
            p_row: 0, 
            p_col: 0, 
            ..Default::default()
        };
        let slice = render_slice(slice_buffer, world_arc, cam, samples_per_pixel, trace_depth, multi_bar);
        slice_buffer.pixels = slice;

        sender.send(slice_buffer).unwrap();
    });

    handle
}


fn main() -> std::io::Result<()>{
    // Image
    let img_aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / img_aspect_ratio) as usize;
    
    let samples_per_pixel = 100;
    let trace_depth: i32 = 5;

    // World
    let mut world = HittableGroup::default();
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0
    )));

    let world_arc = Arc::new(world);

    // Camera
    let cam = Camera::new();

    // Render
    let multi_bar = Arc::new(MultiProgress::new());
    let mut image_canvas = Canvas::new(image_width, image_height);

    // Split into 4 vertical slices and multithread em'
    let slice_width = image_width / 4;
    
    let handle_1 =thread::spawn(|| {
        let mut slice_buffer = SliceBuffer{
            height: image_height, 
            width: image_width, 
            p_row: 0, 
            p_col: 0, 
            ..Default::default()
        };
        let slice = render_slice(slice_buffer, world_arc, cam, samples_per_pixel, trace_depth, multi_bar);
        slice_buffer.pixels = slice;
    });
    let handle_2 =thread::spawn(|| {
        let mut slice_buffer = SliceBuffer{
            height: image_height, 
            width: image_width, 
            p_row: 0, 
            p_col: slice_width * 1, 
            ..Default::default()
        };
        let slice = render_slice(slice_buffer, world_arc, cam, samples_per_pixel, trace_depth, multi_bar);
        slice_buffer.pixels = slice;
    });
    let handle_3 =thread::spawn(|| {
        let mut slice_buffer = SliceBuffer{
            height: image_height, 
            width: image_width, 
            p_row: 0, 
            p_col: slice_width * 2, 
            ..Default::default()
        };
        let slice = render_slice(slice_buffer, world_arc, cam, samples_per_pixel, trace_depth, multi_bar);
        slice_buffer.pixels = slice;
    });
    let handle_4 =thread::spawn(|| {
        let mut slice_buffer = SliceBuffer{
            height: image_height, 
            width: image_width, 
            p_row: 0, 
            p_col: slice_width * 3, 
            ..Default::default()
        };
        let slice = render_slice(slice_buffer, world_arc, cam, samples_per_pixel, trace_depth, multi_bar);
        slice_buffer.pixels = slice;
    });
    
    handle_1.join();
    handle_2.join();
    handle_3.join();
    handle_4.join();

    image_canvas.write_slice(slice_1);
    image_canvas.write_slice(slice_2);
    image_canvas.write_slice(slice_3);
    image_canvas.write_slice(slice_4);
    
    // Render a single slice - single thread basically
    // let mut main_slice = SliceBuffer{height: image_height, width: image_width, ..Default::default()};
    // render_slice(&mut main_slice, world, cam, samples_per_pixel, trace_depth, multi_bar);
    // image_canvas.write_slice(main_slice);
    
    // File
    let mut output_image_file = File::create("output.ppm")?;

    write_img_ppm(image_canvas, &mut output_image_file);
    
    eprintln!("\nDone.");
    Ok(())
}
