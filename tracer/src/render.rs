use std::thread;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

use crate::Color;
use crate::ray_color;
use crate::write_color;
use crate::world::World;
use crate::camera::Camera;
use crate::buffer::{Canvas, SliceBuffer, Pixel};



/// Get tuple specifying how many rows and columns
/// should the canvas be split
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

pub fn render_scene(core_count: usize,
                    scene_width: usize,
                    scene_height: usize,
                    world: Arc<World>,
                    cam: Arc<Camera>,
                    samples_per_pixel: u32,
                    trace_depth: i32,) -> Canvas {
    let (rows, columns) = core_to_slices(core_count);
    let slice_width = scene_width / columns;
    let slice_height = scene_height / rows;

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

    let mut image_canvas = Canvas::new(scene_width, scene_height);
    let multi_bar = Arc::new(MultiProgress::new());
    
    thread::scope(|scope| {
        for row in 0..rows {
            for col in 0..columns {
                    let slice = slices_array[row][col].clone();
                    scope.spawn(|| {
                        render_slice(
                            slice,
                            scene_width,
                            scene_height,
                            world.clone(),
                            cam.clone(),
                            samples_per_pixel,
                            trace_depth,
                            multi_bar.clone()
                        );
                    });
                };
            }
    });

    for row in 0..rows {
        for col in 0..columns {
            let locked_slice_buffer = slices_array[row][col].lock().unwrap();
            let extracted_slice_buffer = locked_slice_buffer.clone(); // Clone the SliceBuffer
    
            image_canvas.write_slice(&extracted_slice_buffer);
        }
    }

    image_canvas
}


fn render_slice(slice_buffer: Arc<Mutex<SliceBuffer>>,
                canvas_width: usize,
                canvas_height: usize,
                world: Arc<World>,
                cam: Arc<Camera>,
                samples_per_pixel: u32,
                trace_depth: i32,
                multi_bar: Arc<MultiProgress>) {
    let mut slice_data = slice_buffer.lock().unwrap();
    let height = slice_data.height;
    let width = slice_data.width;

    let mut slice_vec: Vec<Vec<Pixel>> = Vec::default();

    // Progress bar config
    let height_bar: ProgressBar = multi_bar.add(ProgressBar::new(height as u64));
    height_bar.set_style(ProgressStyle::with_template(
    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta})"
    )
    .unwrap()
    .progress_chars("##-")); 

    for i in 0..height {
        let mut line_buffer: Vec<Pixel> = Vec::default();
        // height - i because the camera renders from the bottom left corner
        let pixel_row = slice_data.p_row + i;

        // Render single line
        for j in 0..width {
            let mut pixel_color = Color::origin();
            let pixel_col = slice_data.p_col + j;

            // Render single pixel
            for _ in 0..samples_per_pixel {
                // Earlier, dividing by slice dimensions caused u,v to be >1.0 which rendered
                // pixels outside the viewport and distorted them a LOT
                // dividing by canvas dimensions fixes that
                let u = (pixel_col as f64 + rand::random::<f64>()) / (canvas_width - 1) as f64;
                let v = (pixel_row as f64 + rand::random::<f64>()) / (canvas_height - 1) as f64;
                let ray = cam.get_ray(u, v);

                pixel_color += ray_color(ray, &world, trace_depth, false);
            }
            line_buffer.push(write_color(pixel_color, samples_per_pixel));
        }
        slice_vec.push(line_buffer);
        height_bar.inc(1);
    }
    height_bar.finish();

    slice_data.pixels = slice_vec;
}
