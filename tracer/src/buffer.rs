
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::Color;
use crate::ray_color;
use crate::write_color;
use crate::color::MAX_COLOR;

use crate::Camera;
use crate::World;

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

#[derive(Default, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

/// Render buffer describing a slice
/// 
/// width, height are the dimensions of the slice
/// p_row, p_col define the absolute position of the slice in the frame
/// pixels is a 2d vector of pixels which will be filled by render_slice
#[derive(Default)]
pub struct SliceBuffer {
    pub width: usize,
    pub height: usize,
    pub p_row: usize,
    pub p_col: usize,
    pub pixels: Vec<Vec<Pixel>>
}

impl SliceBuffer {
    pub fn new_slice(width: usize, height: usize, p_row: usize, p_col: usize) -> SliceBuffer {
        SliceBuffer { width, height, p_row, p_col, ..Default::default() }
    }
}

pub type Canvas = SliceBuffer;
impl Canvas {
    pub fn new(width: usize, height: usize) -> SliceBuffer {
        let mut pixels_array: Vec<Vec<Pixel>> = Vec::with_capacity(height);
        for _ in 0..height {
            let row = vec![Pixel::default(); width];
            pixels_array.push(row);
        }
        SliceBuffer{width, height, p_row: 0, p_col: 0, pixels: pixels_array}
    }

    /// Write a given slice onto the canvas
    pub fn write_slice(&mut self, slice: Arc<Mutex<SliceBuffer>>) -> bool {
        let slice_data = slice.lock().unwrap();
        // Slice is too large
        if slice_data.height > self.height || slice_data.width > self.width {
            return false;
        }
        // Slice position overflows canvas
        if slice_data.p_row + slice_data.height > self.height || slice_data.p_col + slice_data.width > self.width {
            return false;
        }

        for i in 0..slice_data.height {
            for j in 0..slice_data.width {
                self.pixels[slice_data.p_row + i][slice_data.p_col + j] = slice_data.pixels[i][j];
            }
        }

        true
    }
}

pub fn write_img_ppm(canvas: Canvas, file: &mut File) {
    let image_width = canvas.width;
    let image_height = canvas.height;

    let _ = file.write_fmt(format_args!("P3\n{image_width} {image_height}\n{}\n", MAX_COLOR));

    for row in &canvas.pixels {
        for pixel in row {
            let _ = file.write_fmt(format_args!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
        }
    }
}

pub fn render_slice(slice_buffer: Arc<Mutex<SliceBuffer>>,
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

    // TODO: Optimization: initialize slice_vec to the expected size which is known
    let mut slice_vec: Vec<Vec<Pixel>> = Vec::default();

    // Progress bar config
    let height_bar: ProgressBar = multi_bar.add(ProgressBar::new(height as u64));
    height_bar.set_style(ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"
    )
    .unwrap()
    .progress_chars("##-")); 

    for i in (0..height).rev() {
        let mut line_buffer: Vec<Pixel> = Vec::default();
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
            
                pixel_color += ray_color(ray, &world, trace_depth);
            }
            line_buffer.push(write_color(pixel_color, samples_per_pixel));
        }
        slice_vec.push(line_buffer);

        height_bar.set_message(format!("line #{}", pixel_row));
        height_bar.inc(1);
    }
    height_bar.finish();

    slice_data.pixels = slice_vec;
}