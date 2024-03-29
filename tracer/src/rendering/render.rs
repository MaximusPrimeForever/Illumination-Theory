use std::thread;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

use crate::Color;
use crate::graphics::Camera;
use crate::geometry::hittable::{Hittable, HittableSync};
use crate::rendering::{buffer::{Canvas, SliceBuffer}, color::rasterize_color};



/// Return tuple specifying how many rows and columns
/// should the canvas be split into
/// 
/// Return (rows, columns)
fn core_to_slices(core_count: usize) -> (usize, usize) {
    let cores: usize;
    if core_count == 0 { 
        if let Some(avail_cores) = thread::available_parallelism().ok() {
            cores = usize::from(avail_cores);
        } else {
            cores = 1;
        }

    } else {
        cores = core_count;
    }

    let slice_seg = match cores {
        1 => (1, 1),
        2 => (2, 1),
        4 => (2, 2),
        8 => (4, 2),
        12 => (4, 3),
        16 => (4, 4),
        32 => (8, 4),
        64 => (8, 8),
        _ => (1, 1)
    };

    slice_seg
}

/// Render a scene given a World object, and render parameters
/// 
/// Splits the frame into sub-frames according to the given core count, 
/// and render all sub-frames in parallel.
pub fn render_scene(core_count: usize,
                    object: Arc<HittableSync>,
                    cam: Arc<Camera>,
                    samples_per_pixel: usize,
                    trace_depth: usize) -> Canvas {
    let (rows, columns) = core_to_slices(core_count);
    let slice_width = cam.image_width / columns;
    let slice_height = cam.image_height / rows;

    // Generate slices
    let mut slices_array: Vec<Vec<Arc<Mutex<SliceBuffer>>>> = Vec::new();
    for row in 0..rows {
        let mut slices_row: Vec<Arc<Mutex<SliceBuffer>>> = Vec::new();

        for col in 0..columns {
            slices_row.push(Arc::new(Mutex::new(SliceBuffer::new_slice(
                slice_width,
                slice_height,
                row * slice_height,
                col * slice_width
            ))));
        }
        slices_array.push(slices_row)
    }

    let mut image_canvas = Canvas::new(cam.image_width, cam.image_height);
    let multi_bar = Arc::new(MultiProgress::new());

    // Render slices in parallel
    thread::scope(|scope| {
        for row in 0..rows {
            for col in 0..columns {
                    let slice = slices_array[row][col].clone();
                    scope.spawn(|| {
                        render_slice(
                            slice,
                            object.clone(),
                            cam.clone(),
                            samples_per_pixel,
                            trace_depth,
                            multi_bar.clone()
                        );
                    });
                };
            }
    });

    // Assemble slices back into a single canvas
    for row in 0..rows {
        for col in 0..columns {
            let locked_slice_buffer = slices_array[row][col].lock().unwrap();
            let extracted_slice_buffer = locked_slice_buffer.clone();
    
            image_canvas.write_slice(&extracted_slice_buffer);
        }
    }

    image_canvas
}

/// Render a single slice.
/// 
/// Shoots rays into the scene and updates the SliceBuffer with a pixel array.
fn render_slice(slice_buffer: Arc<Mutex<SliceBuffer>>,
                object: Arc<dyn Hittable>,
                cam: Arc<Camera>,
                samples_per_pixel: usize,
                trace_depth: usize,
                multi_bar: Arc<MultiProgress>) {
    let mut slice_data = slice_buffer.lock().unwrap();
    let height = slice_data.pixels.height();
    let width = slice_data.pixels.width();

    // Progress bar config
    let height_bar: ProgressBar = multi_bar.add(ProgressBar::new(height as u64));
    height_bar.set_style(ProgressStyle::with_template(
    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta})"
    )
    .unwrap()
    .progress_chars("##-")); 

    for i in 0..height {
        // height - i because the camera renders from the bottom left corner
        let pixel_row = (slice_data.abs_row_delta + i as usize) as f64;

        // Render single line
        for j in 0..width {
            let mut pixel_color = Color::zero();
            let pixel_col = (slice_data.abs_col_delta + j as usize) as f64;

            // Render single pixel
            for _ in 0..samples_per_pixel {
                let color = cam.render_ray(
                    pixel_row,
                    pixel_col,
                    &object, 
                    trace_depth
                );
                pixel_color += color;
            }
            let pixel = slice_data.pixels.get_pixel_mut(j as u32, i as u32);
            *pixel = rasterize_color(pixel_color, samples_per_pixel);
        }
        // slice_vec.push(line_buffer);
        height_bar.inc(1);
    }
    height_bar.finish();

    // slice_data.pixels = slice_vec;
}
