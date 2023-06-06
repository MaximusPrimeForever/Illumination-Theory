mod vec3;
mod color;

use std::io::{Write, stderr};

// use vec3::Vec3 as Point3;
use vec3::Vec3 as Color;

fn main() {
    // Image
    let img_width: i32 = 256;
    let img_height: i32 = 256;

    // Render

    // Header
    println!("P3\n{img_width} {img_height}\n{}\n", color::MAX_COLOR);

    let img_width_norm: f64 = f64::from(img_width - 1);
    let img_height_norm: f64 = f64::from(img_height - 1);
    let mut tri_start: i32;
    let mut tri_end: i32;
    let mut stderr = stderr();
    
    // Pixels
    for i in 0..img_height {
        tri_start = (img_width / 2) - (i/2);
        tri_end = (img_width / 2) + (i/2);
        eprint!("\rScanlines remaining: {}", img_height-i);
        stderr.flush().unwrap();

        for j in 0..img_width {
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;

            if j >= tri_start && j < tri_end {
                r = f64::from(j) / img_width_norm;
                g = f64::from(img_height - i) / img_height_norm;
                b = 0.25;
            }

            let pixel_color: Color = Color::new(r, g, b);
            color::write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}
