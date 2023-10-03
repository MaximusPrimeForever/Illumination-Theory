/// Render buffer describing a slice
/// 
/// width, height are the dimensions of the slice
/// p_row, p_col define the absolute position of the slice in the frame
/// pixels is a 2d vector of pixels which will be filled by render_slice
#[derive(Clone)]
pub struct SliceBuffer {
    pub width: usize,
    pub height: usize,
    pub abs_row_delta: usize,
    pub abs_col_delta: usize,
    pub pixels: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>
}

impl SliceBuffer {
    pub fn new_slice(width: usize, height: usize, p_row: usize, p_col: usize) -> SliceBuffer {
        SliceBuffer { 
            width,
            height,
            abs_row_delta: p_row,
            abs_col_delta: p_col,
            pixels: image::ImageBuffer::new(
                width as u32,
                height as u32
            )
        }
    }
}

pub type Canvas = SliceBuffer;
impl Canvas {
    pub fn new(width: usize, height: usize) -> SliceBuffer {
        let pixels_array = image::ImageBuffer::new(
            width as u32,
            height as u32
        );
        SliceBuffer{width, height, abs_row_delta: 0, abs_col_delta: 0, pixels: pixels_array}
    }

    /// Write a given slice onto the canvas
    pub fn write_slice(&mut self, slice: &SliceBuffer) -> bool {
        let slice_data = slice;
        // Slice is too large
        if slice_data.height > self.height || slice_data.width > self.width {
            return false;
        }
        // Slice position overflows canvas
        if slice_data.abs_row_delta + slice_data.height > self.height || slice_data.abs_col_delta + slice_data.width > self.width {
            return false;
        }

        for (x, y, pixel) in slice_data.pixels.enumerate_pixels() {
            let canvas_pixel = self.pixels.get_pixel_mut(
                slice_data.abs_col_delta as u32 + x, 
                slice_data.abs_row_delta as u32 + y
            );
            *canvas_pixel = *pixel;
        }

        true
    }

    pub fn save_png(&self, path: &str) {
        self.pixels.save_with_format(path, image::ImageFormat::Png).unwrap()
    }
}

// pub fn write_img_ppm(canvas: Canvas, file: &mut File) {
//     let image_width = canvas.width;
//     let image_height = canvas.height;

//     let _ = file.write_fmt(format_args!("P3\n{image_width} {image_height}\n{}\n", MAX_COLOR));

//     for row in &canvas.pixels {
//         for pixel in row {
//             let _ = file.write_fmt(format_args!("{} {} {}\n", pixel., pixel.g, pixel.b));
//         }
//     }
// }

