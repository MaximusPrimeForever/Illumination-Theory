
use std::fs::File;
use std::io::Write;

use crate::color::MAX_COLOR;

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
#[derive(Default, Clone)]
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
    pub fn write_slice(&mut self, slice: &SliceBuffer) -> bool {
        let slice_data = slice;
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
