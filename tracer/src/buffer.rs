
use std::io::Write;
use std::sync::Arc;
use std::fs::File;

use crate::Color;
use crate::ray_color;
use crate::write_color;
use crate::color::MAX_COLOR;

use crate::Camera;
use crate::HittableGroup;

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
    pub fn write_slice(&mut self, slice: SliceBuffer) -> bool {
        // Slice is too large
        if slice.height > self.height || slice.width > self.width {
            return false;
        }
        // Slice position overflows canvas
        if slice.p_row + slice.height > self.height || slice.p_col + slice.width > self.width {
            return false;
        }

        for i in 0..slice.height {
            for j in 0..slice.width {
                self.pixels[slice.p_row + i][slice.p_col + j] = slice.pixels[i][j];
            }
        }

        true
    }
}

pub fn write_img_ppm(canvas: Canvas, file: &mut File) {
    let image_width = canvas.width;
    let image_height = canvas.height;

    file.write_fmt(format_args!("P3\n{image_width} {image_height}\n{}\n", MAX_COLOR));

    for row in &canvas.pixels {
        for pixel in row {
            file.write_fmt(format_args!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
        }
    }
}

pub fn render_slice(slice_buffer: &mut SliceBuffer, world: HittableGroup, cam: Camera, samples_per_pixel: u32, trace_depth: i32, multi_bar: Arc<MultiProgress>) {
    let height = slice_buffer.height;
    let width = slice_buffer.width;
    let mut slice_vec: Vec<Vec<Pixel>> = Vec::default();

    let height_bar: ProgressBar = multi_bar.add(ProgressBar::new(height as u64));
    let width_bar: ProgressBar = multi_bar.add(ProgressBar::new(width as u64));

    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    ).unwrap().progress_chars("##-");
    height_bar.set_style(sty.clone());
    width_bar.set_style(sty);    

    for i in (0..height).rev() {
        let mut line_buffer: Vec<Pixel> = Vec::default();

        // Render single line
        for j in 0..width {
            let mut pixel_color = Color::origin();

            // Render single pixel
            for _ in 0..samples_per_pixel {
                let u = (j as f64 + rand::random::<f64>()) / (width - 1) as f64;
                let v = (i as f64 + rand::random::<f64>()) / (height - 1) as f64;
                let ray = cam.get_ray(u, v);
            
                pixel_color += ray_color(&ray, &world, trace_depth);
            }
            width_bar.set_message(format!("col #{}", j));
            width_bar.inc(1);

            line_buffer.push(write_color(pixel_color, samples_per_pixel));
        }
        width_bar.finish();
        width_bar.reset();

        slice_vec.push(line_buffer);

        height_bar.set_message(format!("line #{}", height - i));
        height_bar.inc(1);
    }
    height_bar.finish();

    slice_buffer.pixels = slice_vec;
}