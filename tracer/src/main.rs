mod ray;
mod vec3;
mod color;
mod buffer;
mod sphere;
mod camera;
mod hittable;
mod rtweekend;
mod hittable_group;

use std::thread;
use std::rc::{Rc};
use std::fs::File;
use std::sync::Arc;

use sphere::Sphere;
use camera::Camera;
use vec3::{Point3, Color};
use buffer::{SliceBuffer, Canvas, write_img_ppm, render_slice};

use hittable_group::HittableGroup;
use crate::color::{ray_color, write_color};
use crate::vec3::Vec3;

use indicatif::{MultiProgress};

fn main() -> std::io::Result<()>{
    // Image
    let img_aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / img_aspect_ratio) as usize;
    
    let samples_per_pixel = 100;
    let trace_depth: i32 = 5;

    // World
    let mut world = HittableGroup::default();
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
    let mut image_canvas = Canvas::new(image_width, image_height);

    // Split into 4 vertical slices and multithread em'
    let slice_width = image_width / 4;
    let mut slice_1 = SliceBuffer{height: image_height, width: slice_width, p_col: 0, p_row: 0, ..Default::default()};
    let mut slice_2 = SliceBuffer{height: image_height, width: slice_width, p_col: slice_width * 1, p_row: 0, ..Default::default()};
    let mut slice_3 = SliceBuffer{height: image_height, width: slice_width, p_col: slice_width * 2, p_row: 0, ..Default::default()};
    let mut slice_4 = SliceBuffer{height: image_height, width: slice_width, p_col: slice_width * 3, p_row: 0, ..Default::default()};

    let handle_1 =thread::spawn(|| {
        render_slice(&mut slice_1, world, cam, samples_per_pixel, trace_depth, multi_bar);
    });
    let handle_2 =thread::spawn(|| {
        render_slice(&mut slice_2, world, cam, samples_per_pixel, trace_depth, multi_bar);
    });
    let handle_3 =thread::spawn(|| {
        render_slice(&mut slice_3, world, cam, samples_per_pixel, trace_depth, multi_bar);
    });
    let handle_4 =thread::spawn(|| {
        render_slice(&mut slice_4, world, cam, samples_per_pixel, trace_depth, multi_bar);
    });
    // let mut main_slice = SliceBuffer{height: image_height, width: image_width, ..Default::default()};
    // render_slice(&mut main_slice, world, cam, samples_per_pixel, trace_depth, multi_bar);

    handle_1.join();
    handle_2.join();
    handle_3.join();
    handle_4.join();
    
    // File
    let mut output_image_file = File::create("output.ppm")?;

    // image_canvas.write_slice(main_slice);
    image_canvas.write_slice(slice_1);
    image_canvas.write_slice(slice_2);
    image_canvas.write_slice(slice_3);
    image_canvas.write_slice(slice_4);

    write_img_ppm(image_canvas, &mut output_image_file);
    
    eprintln!("\nDone.");
    Ok(())
}
